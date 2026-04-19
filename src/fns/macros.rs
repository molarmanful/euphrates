#[crabtime::function]
fn f_2_to_1(name: String) {
    let f = name.to_lowercase();
    crabtime::output! {
        pub const {{name}}: EuDef = |env| {
            let a1 = env.arg("a1")?;
            let a0 = env.arg("a0")?;
            env.push(a0.{{f}}(a1));
            Ok(())
        };
    }
}

pub(crate) use f_2_to_1;

#[crabtime::function]
fn f_2_try_1(name: String) {
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

pub(crate) use f_2_try_1;
