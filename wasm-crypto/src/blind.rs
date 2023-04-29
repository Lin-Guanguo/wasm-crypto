use std::convert::TryInto;

use crate::utils::log;
use base64::engine::general_purpose::URL_SAFE_NO_PAD as Base64;
use base64::prelude::*;
use num_bigint::BigUint;
use num_modular::*;
use num_prime::nt_funcs::next_prime;
use sha2::{Digest, Sha256};

const KEY_BYTES: usize = 256;
const HASH_BYTES: usize = 24;
const SALT: [u8; 16] = [0; 16];
const COST: u32 = 6;

// r for blind. 2/(ln(2^512)) = 1/177, Means on average 177 tries to find a prime number
const R_BYTES: usize = 512 / 8;

fn hash(password: &[u8]) -> [u8; HASH_BYTES] {
    bcrypt::bcrypt(COST, SALT, password)
}

fn sha256(i: &[u8]) -> [u8; 256 / 8] {
    let mut hasher = Sha256::new();
    hasher.update(i);
    (&hasher.finalize()[..])
        .try_into()
        .expect("sha256 bits error")
}

const fn div_ceil(l: usize, r: usize) -> usize {
    (l + r - 1) / r
}

fn kdf<const N: usize>(password: &str) -> [u8; N] {
    let mut base = [0u8; N];
    let mut password_i = vec![0];
    password_i.extend_from_slice(password.as_bytes());
    for i in 0..div_ceil(N, HASH_BYTES) {
        password_i[0] = i as u8;
        let part = hash(&password_i);
        let l = i * HASH_BYTES;
        let r = ((i + 1) * HASH_BYTES).min(N);
        (&mut base[l..r]).copy_from_slice(&part[..(r - l)]);
    }
    base
}

fn bn_decode(s: &str) -> BigUint {
    BigUint::from_bytes_be(&Base64.decode(s).unwrap())
}

fn bn_encode(n: &BigUint) -> String {
    Base64.encode(n.to_bytes_be())
}

fn get_m(password: &str) -> BigUint {
    let mut m = hash(password.as_bytes()).to_vec();
    m.extend_from_slice(&sha256(&m));
    BigUint::from_bytes_be(&m)
}

fn get_r(password: &str) -> BigUint {
    let base: [u8; R_BYTES] = kdf(password);
    let base = BigUint::from_bytes_be(&base);
    next_prime(&base, None).unwrap()
}

pub fn get_m_encode(order_no: String, phrase: String) -> String {
    bn_encode(&get_m(&(order_no + &phrase)))
}

pub fn get_blind_token(order_no: String, phrase: String, n: String, e: String) -> String {
    let password = order_no + &phrase;
    let n = bn_decode(&n);
    let e = bn_decode(&e);
    let m = get_m(&password);
    let r = get_r(&password);
    log(&format!("wasm get_blind_token: m={}, r={}", m, r));
    let blind_m = m.mulm(r.powm(e, &n), &n);
    log(&format!("wasm get_blind_token: blind_m={}", blind_m));
    return bn_encode(&blind_m);
}

pub fn deblind_sign_token(
    sign_blind_token_encode: String,
    order_no: String,
    phrase: String,
    goods_id: u64,
    n: String,
    e: String,
) -> String {
    let password = order_no + &phrase;
    let n = bn_decode(&n);
    let e = bn_decode(&e);
    let m = get_m(&password);
    let r = get_r(&password);
    let sign_blind_token = bn_decode(&sign_blind_token_encode);
    let goods_id = BigUint::from_bytes_be(&sha256(goods_id.to_string().as_bytes()));
    let r_inv = r.clone().invm(&n).unwrap();
    let sign_token = sign_blind_token.mulm(r_inv, &n);
    let design_token = sign_token.powm(e.clone(), &n);

    if design_token == m.mulm(goods_id, &n) {
        bn_encode(&design_token)
    } else {
        format!(
            "ERROR: sign_blind_token bad format, sign_blind_token={}",
            sign_blind_token_encode
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let password = "hello";
        let m = get_m(password);
        let r = get_m(password);
        println!("m={}, r={}", m, r);
    }
}
