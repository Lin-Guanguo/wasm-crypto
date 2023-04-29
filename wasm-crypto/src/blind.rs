use crate::utils::log;
use base64::engine::general_purpose::URL_SAFE_NO_PAD as Base64;
use base64::prelude::*;
use num_bigint::BigUint;
use num_prime::nt_funcs::next_prime;

const KEY_BYTES: usize = 256;
const HASH_BYTES: usize = 24;
const SALT: [u8; 16] = [0; 16];
const COST: u32 = 6;

// r for blind. 2/(ln(2^512)) = 1/177, Means on average 177 tries to find a prime number
const R_BYTES: usize = 512 / 8;

fn hash(password: &[u8]) -> [u8; HASH_BYTES] {
    bcrypt::bcrypt(COST, SALT, password)
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

fn get_m(password: &str) -> BigUint {
    BigUint::from_bytes_be(&hash(password.as_bytes()))
}

fn get_r(password: &str) -> BigUint {
    let base: [u8; R_BYTES] = kdf(password);
    let base = BigUint::from_bytes_be(&base);
    log(&format!("start to find next_prime of {}", base));
    next_prime(&base, None).unwrap()
}

pub fn blind_token(order_no: String, phrase: String, n: String, e: String) -> String {
    let password = order_no + &phrase;
    let m = get_m(&password);
    let r = get_r(&password);
    log(&format!("wasm: m={}, r={}", m, r));
    return "".to_string();
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