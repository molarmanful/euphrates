use std::{
    error::Error,
    fs,
    io,
    path,
};

use clap::Parser;
use euph::env::EuEnv;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(group = "input")]
    /// Evaluate code from the given file
    file: Option<path::PathBuf>,
    /// Evaluate the given string
    #[arg(short, long, group = "input")]
    string: Option<String>,
    /// Evaluate code passed from STDIN
    #[arg(short = 'i', long, group = "input")]
    stdin: bool,
}

fn main() {
    let cli = Cli::parse();
    if let Some(s) = cli.string {
        run(&s);
    } else if let Some(p) = cli.file {
        run_res(fs::read_to_string(p));
    } else if cli.stdin {
        run_res(io::read_to_string(io::stdin()));
    }
}

fn run_res<E: Error>(r: Result<String, E>) {
    match r {
        Ok(code) => run(&code),
        Err(e) => eprintln!("ERR:\n{e}"),
    };
}

fn run(code: &str) {
    match EuEnv::run_str(code) {
        Ok(env) => println!("{env}"),
        Err(e) => {
            eprintln!("ERR:");
            for c in e.0.chain() {
                eprintln!("{c}");
            }
            std::process::exit(1);
        }
    };
}
