// mod utils;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn string_from_bytes(input: &[u8]) -> String {
    x_toolkit_convert::string::from_bytes(input)
}

#[wasm_bindgen]
pub fn string_to_bytes(input: String) -> Vec<u8> {
    x_toolkit_convert::string::to_bytes(input)
}

#[wasm_bindgen]
pub fn hex_encode(input: Vec<u8>) -> String {
    x_toolkit_convert::hex::encode(input)
}

#[wasm_bindgen]
pub fn hex_decode(input: String) -> Vec<u8> {
    x_toolkit_convert::hex::decode(input)
}

#[wasm_bindgen]
pub fn base58_encode(input: Vec<u8>) -> String {
    x_toolkit_convert::base58::encode(input)
}

#[wasm_bindgen]
pub fn base58_decode(input: String) -> Vec<u8> {
    x_toolkit_convert::base58::decode(input)
}
