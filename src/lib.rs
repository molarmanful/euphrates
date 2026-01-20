#![feature(try_find)]
#![feature(iterator_try_reduce)]
#![feature(trait_alias)]
#![feature(bool_to_result)]
#![feature(iter_intersperse)]

pub mod env;
pub mod fns;
pub mod parser;
pub mod types;
pub mod utils;

use env::EuEnv;

wit_bindgen::generate!();

struct Glue;

impl Guest for Glue {
    fn run_euph(code: String) {
        match EuEnv::run_str(&code) {
            Ok(env) => println!("{env}"),
            Err(e) => {
                eprintln!("ERR:");
                e.0.chain().for_each(|c| eprintln!("{c}"));
            }
        }
    }
}

export!(Glue);
