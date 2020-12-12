// mod utils;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn convert_string_to_bytes(str: String) -> Vec<u8> {
    str.into_bytes()
}

#[wasm_bindgen]
pub fn convert_bytes_to_string(data: &[u8]) -> String {
    std::str::from_utf8(data).unwrap().to_string()
}

#[wasm_bindgen]
pub fn convert_hex_encode(data: Vec<u8>) -> String {
    x_toolkit_convert::hex::encode(data)
}

#[wasm_bindgen]
pub fn convert_hex_decode(data: String) -> Vec<u8> {
    x_toolkit_convert::hex::decode(data).unwrap()
}
