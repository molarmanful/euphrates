use std::{
    fs,
    io,
    path,
};

use clap::Parser;
use euph::{
    EuEnvOpts,
    env::EuEnv,
};

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
    /// Turn on debug mode
    #[arg(long, default_value_t = false)]
    debug: bool,
    /// Print final program state
    #[arg(long, default_value_t = false)]
    dump: bool,
}

fn main() {
    let cli = Cli::parse();
    let opts = EuEnvOpts { debug: cli.debug };

    let res: anyhow::Result<String> = if let Some(s) = cli.string {
        Ok(s)
    } else if let Some(p) = cli.file {
        fs::read_to_string(p).map_err(Into::into)
    } else if cli.stdin {
        io::read_to_string(io::stdin()).map_err(Into::into)
    } else {
        return;
    };

    match res
        .map_err(Into::into)
        .and_then(|code| EuEnv::run_str(&code, &opts))
    {
        Ok(env) => {
            if cli.debug || cli.dump {
                println!("{env}");
            }
        }
        Err(e) => {
            eprintln!("ERR:");
            for c in e.0.chain() {
                eprintln!("{c}");
            }
            std::process::exit(1);
        }
    }
}
