mod blind;
mod utils;

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
pub fn getBlindToken(orderNo: String, phrase: String, n: String, e: String) -> String {
    utils::set_panic_hook();
    blind::get_blind_token(orderNo, phrase, n, e)
}

/**
 * is signBlindToken is right. return "OK"
 */
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn checkSignBlindToken(
    signBlindToken: String,
    orderNo: String,
    phrase: String,
    goodsId: u64,
    n: String,
    e: String,
) -> String {
    utils::set_panic_hook();
    blind::check_sign_blind_token(signBlindToken, orderNo, phrase, goodsId, n, e)
}
