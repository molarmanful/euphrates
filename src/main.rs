use euphrates::env::EuEnv;

const TEST: &str = r#"
(1 2 3 4)#vec >seq dup (a) -> (a +)map
"#;

fn main() {
    match EuEnv::run_str(TEST) {
        Ok(env) => println!("{env}"),
        Err(e) => {
            eprintln!("ERR:");
            e.chain().for_each(|c| eprintln!("{c}"));
            std::process::exit(1);
        }
    }
}
