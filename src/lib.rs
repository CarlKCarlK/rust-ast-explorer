use html_escape::encode_text;
// todo make dual license

use wasm_bindgen::prelude::*;

// See https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
// See https://docs.rs/syn/latest/syn/struct.File.html

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn search(code: &str) -> String {
    let ast_result = syn::parse_str::<syn::File>(code);
    match ast_result {
        Ok(ast) => {
            let debug_string = format!("{:#?}", ast);
            let html = encode_text(&debug_string).into_owned();
            html
        }
        Err(e) => {
            format!("Error: {}", e)
        }
    }
}

#[test]
fn test() {
    let code = "fn main() { println!(); }";
    let result = search(code);
    println!("result='{}'", result);
    // assert_eq!(result, "code='fn main() { println!(); }', result=''");
}
