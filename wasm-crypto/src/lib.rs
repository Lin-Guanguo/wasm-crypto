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
pub fn blindToken(orderNo: String, phrase: String, n: String, e: String) -> String {
    utils::set_panic_hook();
    blind::blind_token(orderNo, phrase, n, e)
}
