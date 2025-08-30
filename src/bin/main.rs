use std::io;

use clap::Parser;
use euphrates::env::EuEnv;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {}

fn main() {
    let _cli = Cli::parse();
    match io::read_to_string(io::stdin()) {
        Ok(code) => match EuEnv::run_str(&code) {
            Ok(env) => println!("{env}"),
            Err(e) => {
                eprintln!("ERR:");
                e.0.chain().for_each(|c| eprintln!("{c}"));
                std::process::exit(1);
            }
        },
        Err(e) => eprintln!("ERR:\n{e}"),
    }
}
