use std::sync::{
    Arc,
    atomic::AtomicBool,
};
#[cfg(not(target_arch = "wasm32"))]
use std::{
    fs,
    io,
    path,
    sync::atomic::Ordering,
};

#[cfg(not(target_arch = "wasm32"))]
use clap::Parser;
use euph::{
    EuEnvOpts,
    env::{
        EuEnv,
        EuEnvCtx,
    },
};
use imbl::GenericHashMap;
#[cfg(not(target_arch = "wasm32"))]
use rustyline::{
    DefaultEditor,
    error::ReadlineError,
};

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let cli = Cli::parse();

    let res: anyhow::Result<String> = if let Some(s) = cli.string {
        Ok(s)
    } else if let Some(p) = cli.file {
        fs::read_to_string(p).map_err(Into::into)
    } else if cli.stdin {
        io::read_to_string(io::stdin()).map_err(Into::into)
    } else {
        if let Err(e) = repl() {
            eprintln!("ERR:\n{e}");
        }
        return;
    };

    let ctx = EuEnvCtx::new(
        EuEnvOpts { debug: cli.debug },
        Arc::new(AtomicBool::new(true)),
        rand::rng(),
    );

    match res
        .map_err(Into::into)
        .and_then(|code| EuEnv::apply_str(&code, &[], GenericHashMap::new(), &ctx))
    {
        Ok(env) => {
            if cli.debug || cli.dump {
                println!("<< {env}");
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

#[cfg(not(target_arch = "wasm32"))]
fn repl() -> anyhow::Result<()> {
    let mut rl = DefaultEditor::new()?;

    let interrupt = Arc::new(AtomicBool::new(true));
    let i = interrupt.clone();
    ctrlc::set_handler(move || {
        i.store(false, Ordering::SeqCst);
    })?;

    let ctx = EuEnvCtx::new(EuEnvOpts { debug: false }, interrupt, rand::rng());
    let mut env = EuEnv::new([], &[], GenericHashMap::new(), &ctx);

    loop {
        match rl.readline("euph> ") {
            Ok(code) => match EuEnv::apply_str(&code, &[], env.scope.clone(), env.ctx) {
                Ok(res) => {
                    env = res;
                    println!("{env}");
                }
                Err(e) => {
                    eprintln!("ERR:");
                    for c in e.0.chain() {
                        eprintln!("{c}");
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(e) => {
                eprintln!("ERR:\n{e:?}");
                break;
            }
        }
    }

    Ok(())
}
