mod fns;
mod parser;
mod state;
mod types;
mod utils;
use std::fmt;

use wasm_bindgen::prelude::*;

use crate::state::EuState;

type EvalResult<'st> = Result<EuState<'st>, EvalError<'st>>;
type EvalOption<'e> = Option<EvalError<'e>>;
type EvalError<'e> = Box<dyn fmt::Display + 'e>;

#[wasm_bindgen]
pub fn run() -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x() {
        match EuState::run("1 2 3 4") {
            Ok(st) => println!("{}", st),
            Err(e) => panic!("{}", e),
        }
        panic!();
    }
}
