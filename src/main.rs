use euphrates::env::EuEnv;

const TEST: &str = r#"
(1 2 3 4)#vec
"#;

fn main() {
    let mut env = EuEnv::new();
    if let Err(e) = env.eval_str(TEST) {
        eprintln!("ERR:");
        e.chain().for_each(|c| eprintln!("{c}"));
        std::process::exit(1);
    }
    println!("{}", env);
}
