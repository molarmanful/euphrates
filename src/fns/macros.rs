#[crabtime::function]
fn f_2_to_1(name: String) {
    let f = name.to_lowercase();
    crabtime::output! {
        pub const {{name}}: EuDef = |env| {
            let a1 = env.arg("a1")?;
            let a0 = env.arg("a0")?;
            env.push(a0.{{f}}(a1)?);
            Ok(())
        };
    }
}

pub(crate) use f_2_to_1;

#[crabtime::function]
fn f_env_2_to_1(name: String) {
    let f = format!("{}_env", name.to_lowercase());
    crabtime::output! {
        pub const {{name}}: EuDef = |env| {
            let a1 = env.arg("a1 (eval)")?;
            let a0 = env.arg("a0")?;
            env.push(a0.{{f}}(a1, env.scope.clone(), env.ctx)?);
            Ok(())
        };
    }
}

pub(crate) use f_env_2_to_1;

#[crabtime::function]
fn f_env_3_to_1(name: String, a1: String) {
    let f = format!("{}_env", name.to_lowercase());
    let a1 = format!(r#""a1{a1}""#);
    crabtime::output! {
        pub const {{name}}: EuDef = |env| {
            let a2 = env.arg("a2 (eval)")?;
            let a1 = env.arg({{a1}})?;
            let a0 = env.arg("a0")?;
            env.push(a0.{{f}}(a1, a2, env.scope.clone(), env.ctx)?);
            Ok(())
        };
    }
}

pub(crate) use f_env_3_to_1;
