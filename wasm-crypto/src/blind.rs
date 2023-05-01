use std::convert::TryInto;

use crate::utils::{console_log, CryptoError, CryptoResult, CryptoResultSure};
use base64::engine::general_purpose::URL_SAFE_NO_PAD as Base64;
use base64::prelude::*;
use num_bigint::BigUint;
use num_modular::*;
use num_prime::nt_funcs::next_prime;

const M_BYTES: usize = 1024 / 8;
// r for blind. 2/(ln(2^512)) = 1/177, Means on average 177 tries to find a prime number
const R_BYTES: usize = 512 / 8;
const HASH_BYTES: usize = 256 / 8;

fn sha256(i: &[u8]) -> CryptoResult<[u8; HASH_BYTES]> {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(i);
    (&hasher.finalize()[..]).try_into().sure()
}

fn kdf(pwd: impl AsRef<[u8]>, salt: impl AsRef<[u8]>, len: usize) -> CryptoResult<Vec<u8>> {
    let mut config = argon2::Config::default();
    config.hash_length = len as u32;
    argon2::hash_raw(pwd.as_ref(), salt.as_ref(), &config).sure()
}

fn bn_decode(s: &str) -> CryptoResult<BigUint> {
    Ok(BigUint::from_bytes_be(&Base64.decode(s).sure()?))
}

fn bn_encode(n: &BigUint) -> CryptoResult<String> {
    Ok(Base64.encode(n.to_bytes_be()))
}

fn get_m(phrase: &str, order_no: &str) -> CryptoResult<BigUint> {
    let mut m = kdf(phrase, order_no, M_BYTES - HASH_BYTES)?;
    m.extend_from_slice(&sha256(&m)?);
    Ok(BigUint::from_bytes_be(&m))
}

fn get_r(phrase: &str, order_no: &str) -> CryptoResult<BigUint> {
    let base = kdf(phrase, order_no, R_BYTES)?;
    let base = BigUint::from_bytes_be(&base);
    next_prime(&base, None).ok_or(CryptoError::NextPrime(base))
}

pub fn get_m_encode(order_no: String, phrase: String) -> CryptoResult<String> {
    bn_encode(&get_m(&phrase, &order_no)?)
}

pub fn get_blind_token(
    order_no: String,
    phrase: String,
    n: String,
    e: String,
) -> CryptoResult<String> {
    let n = bn_decode(&n)?;
    let e = bn_decode(&e)?;
    let m = get_m(&phrase, &order_no)?;
    let r = get_r(&phrase, &order_no)?;
    console_log(&format!("wasm get_blind_token: m={}, r={}", m, r));
    let blind_m = m.mulm(r.powm(e, &n), &n);
    console_log(&format!("wasm get_blind_token: blind_m={}", blind_m));
    bn_encode(&blind_m)
}

pub fn deblind_sign_token(
    sign_blind_token_encode: String,
    order_no: String,
    phrase: String,
    goods_id: u64,
    n: String,
    e: String,
) -> CryptoResult<String> {
    let n = bn_decode(&n)?;
    let e = bn_decode(&e)?;
    let m = get_m(&phrase, &order_no)?;
    let r = get_r(&phrase, &order_no)?;
    let sign_blind_token = bn_decode(&sign_blind_token_encode)?;
    let goods_id = BigUint::from_bytes_be(&sha256(goods_id.to_string().as_bytes())?);
    let r_inv = r.clone().invm(&n).unwrap();
    let sign_token = sign_blind_token.mulm(r_inv, &n);
    let design_token = sign_token.powm(e.clone(), &n);

    if design_token == m.mulm(goods_id, &n) {
        bn_encode(&design_token)
    } else {
        Err(CryptoError::SignTokenIllegal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
