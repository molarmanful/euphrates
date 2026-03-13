use crate::{
    fns::EuDef,
    types::EuType,
};

pub const CMP: EuDef = |env| {
    let a1 = env.arg("a1")?;
    let a0 = env.arg("a0")?;
    env.push(EuType::i32(a0.cmp(&a1) as i32));
    Ok(())
};

macro_rules! cmp_binop {
    ($name:ident, $op:tt) => {
        pub const $name: EuDef = |env| {
            let a1 = env.arg("a1")?;
            let a0 = env.arg("a0")?;
            env.push(EuType::Bool(a0 $op a1));
            Ok(())
        };
    };
}

cmp_binop!(EQ, ==);
cmp_binop!(NE, !=);
cmp_binop!(LT, <);
cmp_binop!(LE, <=);
cmp_binop!(GT, >);
cmp_binop!(GE, >=);
