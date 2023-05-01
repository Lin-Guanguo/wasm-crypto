use std::error::Error;

use num_bigint::BigUint;
use wasm_bindgen::JsValue;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn console_log(s: &str) {
    super::log(s)
}

pub type CryptoResult<T> = Result<T, CryptoError>;

#[derive(thiserror::Error, Debug)]
pub enum CryptoError {
    #[error("unexpected error, cause by {0}")]
    Unexpected(#[from] Box<dyn std::error::Error>),

    #[error("find next prime error for {0}")]
    NextPrime(BigUint),

    #[error("sign token illegal")]
    SignTokenIllegal(),
}

impl Into<JsValue> for CryptoError {
    fn into(self) -> JsValue {
        format!("CryptoError(wasm): {}", self).into()
    }
}

pub trait CryptoResultSure<T> {
    fn sure(self) -> CryptoResult<T>;
}

impl<T, E: std::error::Error + 'static> CryptoResultSure<T> for Result<T, E> {
    fn sure(self) -> CryptoResult<T> {
        self.map_err(|e| CryptoError::Unexpected(Box::new(e)))
    }
}
