use euphrates::env::EuEnv;

const TEST: &str = r#"
(1 2 3 4)#vec ((1+) (1-))#vec map
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
