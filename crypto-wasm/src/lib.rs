// mod utils;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn digest_sha256(input: &[u8]) -> Vec<u8> {
    x_toolkit_crypto::digest::sha256(input).to_vec()
}

#[wasm_bindgen]
pub fn digest_ripemd160(input: &[u8]) -> Vec<u8> {
    x_toolkit_crypto::digest::ripemd160(input).to_vec()
}
