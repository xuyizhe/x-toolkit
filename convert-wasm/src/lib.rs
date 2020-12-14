// mod utils;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn string_from_bytes(data: &[u8]) -> String {
    x_toolkit_convert::string::from_bytes(data)
}

#[wasm_bindgen]
pub fn string_to_bytes(data: String) -> Vec<u8> {
    x_toolkit_convert::string::to_bytes(data)
}

#[wasm_bindgen]
pub fn hex_encode(data: Vec<u8>) -> String {
    x_toolkit_convert::hex::encode(data)
}

#[wasm_bindgen]
pub fn hex_decode(data: String) -> Vec<u8> {
    x_toolkit_convert::hex::decode(data).unwrap()
}
