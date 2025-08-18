#![feature(try_find)]

pub mod env;
pub mod fns;
pub mod parser;
pub mod types;
pub mod utils;

use env::EuEnv;

pub fn wasm(code: &str) {
    match EuEnv::run_str(code) {
        Ok(env) => println!("{env}"),
        Err(e) => {
            eprintln!("ERR:");
            e.0.chain().for_each(|c| eprintln!("{c}"));
        }
    }
}
