use crate::{
    fns::{
        EuDef,
        macros::f_2_to_1,
    },
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

pub const LOOSE_CMP: EuDef = |env| {
    let a1 = env.arg("a1")?;
    let a0 = env.arg("a0")?;
    env.push(EuType::i32(a0.loose_cmp(&a1) as i32));
    Ok(())
};

macro_rules! loose_cmp_binop {
    ($name:ident, $check:ident) => {
        pub const $name: EuDef = |env| {
            let a1 = env.arg("a1")?;
            let a0 = env.arg("a0")?;
            env.push(EuType::Bool(a0.loose_cmp(&a1).$check()));
            Ok(())
        };
    };
}

pub const LOOSE_EQ: EuDef = |env| {
    let a1 = env.arg("a1")?;
    let a0 = env.arg("a0")?;
    env.push(EuType::Bool(a0.loose_eq(&a1)));
    Ok(())
};

pub const LOOSE_NE: EuDef = |env| {
    let a1 = env.arg("a1")?;
    let a0 = env.arg("a0")?;
    env.push(EuType::Bool(!a0.loose_eq(&a1)));
    Ok(())
};

loose_cmp_binop!(LOOSE_LT, is_lt);
loose_cmp_binop!(LOOSE_LE, is_le);
loose_cmp_binop!(LOOSE_GT, is_gt);
loose_cmp_binop!(LOOSE_GE, is_ge);

f_2_to_1!(MIN);
f_2_to_1!(MAX);

pub const LOOSE_MIN: EuDef = |env| {
    let a1 = env.arg("a1")?;
    let a0 = env.arg("a0")?;
    env.push(if a0.loose_cmp(&a1).is_le() { a0 } else { a1 });
    Ok(())
};

pub const LOOSE_MAX: EuDef = |env| {
    let a1 = env.arg("a1")?;
    let a0 = env.arg("a0")?;
    env.push(if a0.loose_cmp(&a1).is_ge() { a0 } else { a1 });
    Ok(())
};
