use euphrates::env::EuEnv;

const TEST: &str = r#"
SeqN0 (1 0/)?tk >vec
"#;

fn main() {
    match EuEnv::run_str(TEST) {
        Ok(env) => println!("{env}"),
        Err(e) => {
            eprintln!("ERR:");
            e.0.chain().for_each(|c| eprintln!("{c}"));
            std::process::exit(1);
        }
    }
}
