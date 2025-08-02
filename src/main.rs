use euphrates::env::EuEnv;

const TEST: &str = r#"
(1 2 3 4)>seq dup + >vec
"#;

fn main() {
    match EuEnv::run_str(TEST) {
        Ok(env) => println!("{}", env),
        Err(e) => {
            eprintln!("ERR:");
            e.chain().for_each(|c| eprintln!("{c}"));
            std::process::exit(1);
        }
    }
}
