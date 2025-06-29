use winnow::Parser;
mod parser;
mod state;
mod types;
mod utils;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() -> String {
    format!("{:?}", parser::euphrates.parse_next(&mut "asdf"))
}
