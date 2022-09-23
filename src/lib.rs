use html_escape::encode_text;
// cmk make dual license
use syn::{
    fold::{self, Fold},
    Expr, ItemFn, Stmt,
};
use wasm_bindgen::prelude::*;

// See https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
// See https://docs.rs/syn/latest/syn/struct.File.html

struct Args {
    result: String,
}

impl Fold for Args {
    fn fold_expr(&mut self, e: Expr) -> Expr {
        self.result += format!("{:?}", e).as_str();
        e
    }

    fn fold_stmt(&mut self, s: Stmt) -> Stmt {
        match s {
            Stmt::Local(s) => {
                // if s.init.is_some() && self.should_print_pat(&s.pat) {
                //     self.let_and_print(s)
                // } else {
                //     Stmt::Local(fold::fold_local(self, s))
                // }
                self.result += "Local";
                Stmt::Local(s)
            }
            _ => fold::fold_stmt(self, s),
        }
    }
}

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
    let item_fn = syn::parse_str::<ItemFn>(code).expect("parse_str fails cmk");
    let result = format!("{:#?}", item_fn);
    let html = encode_text(&result).into_owned();

    // let mut args = Args {
    //     result: "".to_string(),
    // };
    // let _output = args.fold_item_fn(item_fn);

    html
}

// cmk return some type of nice error as string

#[test]
fn test() {
    let code = "fn main() { println!(); }";
    let result = search(code);
    println!("result='{}'", result);
    // assert_eq!(result, "code='fn main() { println!(); }', result=''");
}
