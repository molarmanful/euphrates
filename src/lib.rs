mod env;
mod fns;
mod parser;
mod types;
mod utils;

use hipstr::HipStr;
use wasm_bindgen::prelude::*;

type EvalOption<'e> = Option<EvalError<'e>>;

type EvalError<'e> = HipStr<'e>;

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
            1 (1+) "1234.5usize">expr
            "#,
        ) {
            panic!("{e}");
        };
        println!("{}", env.x);
        panic!();
    }
}
