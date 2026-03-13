#[crabtime::function]
fn f_2_to_1(name: String) {
    let f = name.to_lowercase();
    crabtime::output! {
        pub const {{name}}: EuDef = |env| {
            env.check_nargs(2)?;
            let a1 = env.stack.pop().unwrap();
            let a0 = env.stack.pop().unwrap();
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
            env.check_nargs(2)?;
            let a1 = env.stack.pop().unwrap();
            let a0 = env.stack.pop().unwrap();
            env.push(a0.{{f}}(a1, env.scope.clone())?);
            Ok(())
        };
    }
}

pub(crate) use f_env_2_to_1;

#[crabtime::function]
fn f_env_3_to_1(name: String) {
    let f = format!("{}_env", name.to_lowercase());
    crabtime::output! {
        pub const {{name}}: EuDef = |env| {
            env.check_nargs(3)?;
            let a2 = env.stack.pop().unwrap();
            let a1 = env.stack.pop().unwrap();
            let a0 = env.stack.pop().unwrap();
            env.push(a0.{{f}}(a1, a2, env.scope.clone())?);
            Ok(())
        };
    }
}

pub(crate) use f_env_3_to_1;
