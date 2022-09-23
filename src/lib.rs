// cmk make dual license
use wasm_bindgen::prelude::*;

// See https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn search(_end: usize) -> String {
    // table.len < ((end-1)^5*4)^(1/5) = (end-1)*4^(1/5) < (end-1)*1.32
    "Hello, world2!".to_string()
}
