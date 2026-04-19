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
        macros::f_2_to_1,
    },
    types::EuType,
};

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

f_2_to_1!(ADD);
f_2_to_1!(SUB);
f_2_to_1!(MUL);
f_2_to_1!(DIV);
f_2_to_1!(REM);
f_2_to_1!(POW);

pub const DIV_REM: EuDef = |env| {
    let a1 = env.arg("a1")?;
    let a0 = env.arg("a0")?;
    env.push((a0.clone() / a1.clone())?);
    env.push((a0 % a1)?);
    Ok(())
};

pub const SQRT: EuDef = |env| {
    let a0 = env.arg("a0")?.try_f64()?;
    env.push(EuType::f64(a0.sqrt()));
    Ok(())
};

pub const CBRT: EuDef = |env| {
    let a0 = env.arg("a0")?.try_f64()?;
    env.push(EuType::f64(a0.cbrt()));
    Ok(())
};

pub const SIN_COS: EuDef = |env| {
    let a0 = env.arg("a0")?.try_f64()?;
    let (b0, b1) = a0.sin_cos();
    env.push(EuType::f64(b0));
    env.push(EuType::f64(b1));
    Ok(())
};

pub const SIN: EuDef = |env| {
    let a0 = env.arg("a0")?.try_f64()?;
    env.push(EuType::f64(a0.sin()));
    Ok(())
};

pub const COS: EuDef = |env| {
    let a0 = env.arg("a0")?.try_f64()?;
    env.push(EuType::f64(a0.cos()));
    Ok(())
};

pub const TAN: EuDef = |env| {
    let a0 = env.arg("a0")?.try_f64()?;
    env.push(EuType::f64(a0.tan()));
    Ok(())
};

pub const ASIN: EuDef = |env| {
    let a0 = env.arg("a0")?.try_f64()?;
    env.push(EuType::f64(a0.asin()));
    Ok(())
};

pub const ACOS: EuDef = |env| {
    let a0 = env.arg("a0")?.try_f64()?;
    env.push(EuType::f64(a0.acos()));
    Ok(())
};

pub const ATAN: EuDef = |env| {
    let a0 = env.arg("a0")?.try_f64()?;
    env.push(EuType::f64(a0.atan()));
    Ok(())
};

pub const ATAN2: EuDef = |env| {
    let a1 = env.arg("a1")?.try_f64()?;
    let a0 = env.arg("a0")?.try_f64()?;
    env.push(EuType::f64(a0.atan2(a1)));
    Ok(())
};

pub const SINH: EuDef = |env| {
    let a0 = env.arg("a0")?.try_f64()?;
    env.push(EuType::f64(a0.sinh()));
    Ok(())
};

pub const COSH: EuDef = |env| {
    let a0 = env.arg("a0")?.try_f64()?;
    env.push(EuType::f64(a0.cosh()));
    Ok(())
};

pub const TANH: EuDef = |env| {
    let a0 = env.arg("a0")?.try_f64()?;
    env.push(EuType::f64(a0.tanh()));
    Ok(())
};

pub const ASINH: EuDef = |env| {
    let a0 = env.arg("a0")?.try_f64()?;
    env.push(EuType::f64(a0.asinh()));
    Ok(())
};

pub const ACOSH: EuDef = |env| {
    let a0 = env.arg("a0")?.try_f64()?;
    env.push(EuType::f64(a0.acosh()));
    Ok(())
};

pub const ATANH: EuDef = |env| {
    let a0 = env.arg("a0")?.try_f64()?;
    env.push(EuType::f64(a0.atanh()));
    Ok(())
};
