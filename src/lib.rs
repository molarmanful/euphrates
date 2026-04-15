#![feature(try_find)]
#![feature(iterator_try_reduce)]
#![feature(iterator_try_collect)]
#![feature(trait_alias)]
#![feature(bool_to_result)]
#![feature(iter_intersperse)]
#![allow(clippy::same_length_and_capacity)]
// FIXME
#![allow(clippy::missing_errors_doc)]

pub mod env;
pub mod fns;
pub mod parser;
pub mod types;
pub mod utils;

use std::sync::{
    Arc,
    atomic::AtomicBool,
};

use env::EuEnv;

use crate::env::EuEnvCtx;

wit_bindgen::generate!();

struct Glue;

impl Guest for Glue {
    fn run_euph(code: String, opts: EuEnvOpts) {
        let ctx = EuEnvCtx {
            opts,
            interrupt: Arc::new(AtomicBool::new(true)),
        };

        match EuEnv::apply_str(&code, &[], imbl::GenericHashMap::new(), &ctx) {
            Ok(env) => println!("{env}"),
            Err(e) => {
                eprintln!("ERR:");
                e.0.chain().for_each(|c| eprintln!("{c}"));
            }
        }
    }
}

export!(Glue);
