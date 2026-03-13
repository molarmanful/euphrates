use std::ops::{
    Add,
    Div,
    Mul,
    Rem,
    Sub,
};

use num_traits::Pow;
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
