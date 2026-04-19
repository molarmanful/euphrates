use std::ops::{
    Add,
    Div,
    Mul,
    Rem,
    Sub,
};

use num_traits::{
    FloatConst,
    Pow,
};
use ordered_float::{
    FloatCore,
    OrderedFloat,
};

use crate::{
    fns::{
        EuDef,
        macros::f_2_try_1,
    },
    types::EuType,
};

#[crabtime::function]
fn f64_f_1_to_1(name: String) {
    let f = name.to_lowercase();
    crabtime::output! {
        pub const {{name}}: EuDef = |env| {
            let a0 = env.arg("a0")?.try_f64()?;
            env.push(EuType::f64(a0.{{f}}()));
            Ok(())
        };
    }
}

#[crabtime::function]
fn f64_f_2_to_1(name: String) {
    let f = name.to_lowercase();
    crabtime::output! {
        pub const {{name}}: EuDef = |env| {
            let a1 = env.arg("a1")?.try_f64()?;
            let a0 = env.arg("a0")?.try_f64()?;
            env.push(EuType::f64(a0.{{f}}(a1)));
            Ok(())
        };
    }
}

#[crabtime::function]
fn f64_f_1_to_2(name: String) {
    let f = name.to_lowercase();
    crabtime::output! {
        pub const {{name}}: EuDef = |env| {
            let a0 = env.arg("a0")?.try_f64()?;
            let (b0, b1) = a0.{{f}}();
            env.push(EuType::f64(b0));
            env.push(EuType::f64(b1));
            Ok(())
        };
    }
}

pub const MIN_I32: EuDef = |env| {
    env.push(EuType::I32(i32::MIN));
    Ok(())
};

pub const MAX_I32: EuDef = |env| {
    env.push(EuType::I32(i32::MAX));
    Ok(())
};

pub const MIN_I64: EuDef = |env| {
    env.push(EuType::I64(i64::MIN));
    Ok(())
};

pub const MAX_I64: EuDef = |env| {
    env.push(EuType::I64(i64::MAX));
    Ok(())
};

pub const MIN_F64: EuDef = |env| {
    env.push(EuType::F64(OrderedFloat::min_value()));
    Ok(())
};

pub const MAX_F64: EuDef = |env| {
    env.push(EuType::F64(OrderedFloat::max_value()));
    Ok(())
};

pub const INF: EuDef = |env| {
    env.push(EuType::F64(OrderedFloat::infinity()));
    Ok(())
};

pub const NAN: EuDef = |env| {
    env.push(EuType::F64(OrderedFloat::nan()));
    Ok(())
};

pub const PI: EuDef = |env| {
    env.push(EuType::F64(OrderedFloat::PI()));
    Ok(())
};

pub const E: EuDef = |env| {
    env.push(EuType::F64(OrderedFloat::E()));
    Ok(())
};

pub const EPSILON: EuDef = |env| {
    env.push(EuType::F64(OrderedFloat::epsilon()));
    Ok(())
};

pub const TO_I32: EuDef = |env| {
    let a0 = env.arg("a0")?;
    env.push(EuType::opt(a0.to_i32().map(EuType::i32)));
    Ok(())
};

pub const TO_I64: EuDef = |env| {
    let a0 = env.arg("a0")?;
    env.push(EuType::opt(a0.to_i64().map(EuType::i64)));
    Ok(())
};

pub const TO_F64: EuDef = |env| {
    let a0 = env.arg("a0")?;
    env.push(EuType::opt(a0.to_f64().map(EuType::f64)));
    Ok(())
};

pub const TO_IBIG: EuDef = |env| {
    let a0 = env.arg("a0")?;
    env.push(EuType::opt(a0.to_ibig().map(EuType::ibig)));
    Ok(())
};

pub const NEG: EuDef = |env| {
    let a0 = env.arg("a0")?;
    env.push((-a0)?);
    Ok(())
};

f_2_try_1!(ADD);
f_2_try_1!(SUB);
f_2_try_1!(MUL);
f_2_try_1!(DIV);
f_2_try_1!(REM);
f_2_try_1!(POW);

pub const DIV_REM: EuDef = |env| {
    let a1 = env.arg("a1")?;
    let a0 = env.arg("a0")?;
    env.push((a0.clone() / a1.clone())?);
    env.push((a0 % a1)?);
    Ok(())
};

f64_f_1_to_1!(SQRT);
f64_f_1_to_1!(CBRT);
f64_f_1_to_2!(SIN_COS);
f64_f_1_to_1!(SIN);
f64_f_1_to_1!(COS);
f64_f_1_to_1!(TAN);
f64_f_1_to_1!(ASIN);
f64_f_1_to_1!(ACOS);
f64_f_1_to_1!(ATAN);
f64_f_2_to_1!(ATAN2);
f64_f_1_to_1!(SINH);
f64_f_1_to_1!(COSH);
f64_f_1_to_1!(TANH);
f64_f_1_to_1!(ASINH);
f64_f_1_to_1!(ACOSH);
f64_f_1_to_1!(ATANH);
