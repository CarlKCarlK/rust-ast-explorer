// cmk make dual license
use syn::{
    fold::{self, Fold},
    Expr, ItemFn, Stmt,
};
use wasm_bindgen::prelude::*;

// See https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

struct Args {
    result: String,
}

impl Fold for Args {
    fn fold_expr(&mut self, e: Expr) -> Expr {
        match e {
            Expr::Assign(e) => {
                // if self.should_print_expr(&e.left) {
                //     self.assign_and_print(*e.left, &e.eq_token, *e.right)
                // } else {
                //     Expr::Assign(fold::fold_expr_assign(self, e))
                // }
                self.result += "Assign";
                Expr::Assign(e)
            }
            Expr::AssignOp(e) => {
                // if self.should_print_expr(&e.left) {
                //     self.assign_and_print(*e.left, &e.op, *e.right)
                // } else {
                //     Expr::AssignOp(fold::fold_expr_assign_op(self, e))
                // }
                self.result += "AssignOp";
                Expr::AssignOp(e)
            }
            _ => fold::fold_expr(self, e),
        }
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

    let mut args = Args {
        result: "".to_string(),
    };
    let _output = args.fold_item_fn(item_fn);

    args.result
}

#[test]
fn test() {
    let code = r#"
    fn main() {
        let a = 1;
        a = 2;
        a += 3;
    }
    "#;
    let result = search(code);
    assert_eq!(result, "LocalAssignAssignOp");
}
