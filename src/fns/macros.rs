macro_rules! f_2_to_1 {
    ($f:ident) => {
        |env| {
            let a1 = env.arg("a1")?;
            let a0 = env.arg("a0")?;
            env.push(a0.$f(a1));
            Ok(())
        }
    };
}

pub(crate) use f_2_to_1;

macro_rules! f_2_to_try_1 {
    ($f:ident) => {
        |env| {
            let a1 = env.arg("a1")?;
            let a0 = env.arg("a0")?;
            env.push(a0.$f(a1)?);
            Ok(())
        }
    };
}

pub(crate) use f_2_to_try_1;
