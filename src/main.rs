use euphrates::env::EuEnv;

const TEST: &str = r#"
1Some 2 (+ dup)scan
(1)#vec 2 (+ dup)scan
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
