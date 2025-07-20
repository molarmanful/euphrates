mod env;
mod fns;
mod parser;
mod types;
mod utils;

use ecow::EcoString;
use wasm_bindgen::prelude::*;

type EvalOption = Option<EvalError>;

type EvalError = EcoString;

#[wasm_bindgen]
pub fn run() -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::env::EuEnv;

    #[test]
    fn test_x() {
        let mut env = EuEnv::new();
        if let Some(e) = env.eval_str(
            r#"
            1 (2dup)#
            "#,
        ) {
            panic!("{e}");
        };
        println!("{}", env.x);
        panic!();
    }
}
