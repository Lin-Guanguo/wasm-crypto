mod blind;
mod utils;

use crate::utils::CryptoResult;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-crypto!");
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn getBlindToken(
    orderNo: String,
    phrase: String,
    n: String,
    e: String,
) -> CryptoResult<String> {
    utils::set_panic_hook();
    blind::get_blind_token(orderNo, phrase, n, e)
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn deblindSignToken(
    signBlindToken: String,
    orderNo: String,
    phrase: String,
    goodsId: u64,
    n: String,
    e: String,
) -> CryptoResult<String> {
    utils::set_panic_hook();
    blind::deblind_sign_token(signBlindToken, orderNo, phrase, goodsId, n, e)
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn getToken(orderNo: String, phrase: String) -> CryptoResult<String> {
    utils::set_panic_hook();
    blind::get_m_encode(orderNo, phrase)
}
