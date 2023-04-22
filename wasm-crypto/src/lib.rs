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
}

use num_bigint::BigUint;
use num_modular::ModularUnaryOps;

#[wasm_bindgen]
pub fn greet() {
    let a = BigUint::from(11u32);
    let m = BigUint::from(17u32);
    let inv = a.invm(&m);
    alert(&format!("Hello, wasm-crypto! {:?}", inv));
}

#[wasm_bindgen]
pub fn greet2() {
    alert("Hello2, wasm-crypto! version4");
}
