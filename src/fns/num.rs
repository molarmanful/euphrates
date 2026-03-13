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
    fns::EuDef,
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
    let a0 = env.pop()?;
    env.push(EuType::opt(a0.to_i32().map(EuType::i32)));
    Ok(())
};

pub const TO_I64: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::opt(a0.to_i64().map(EuType::i64)));
    Ok(())
};

pub const TO_F64: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::opt(a0.to_f64().map(EuType::f64)));
    Ok(())
};

pub const TO_IBIG: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::opt(a0.to_ibig().map(EuType::ibig)));
    Ok(())
};

pub const NEG: EuDef = |env| {
    let a0 = env.pop()?;
    env.push((-a0)?);
    Ok(())
};

#[crabtime::function]
fn gen_fn_math_binops() {
    for name in ["ADD", "SUB", "MUL", "DIV", "REM", "POW"] {
        let op = name.to_lowercase();
        crabtime::output! {
            pub const {{name}}: EuDef = |env| {
                env.check_nargs(2)?;
                let a1 = env.stack.pop().unwrap();
                let a0 = env.stack.pop().unwrap();
                env.push(a0.{{op}}(a1)?);
                Ok(())
            };
        }
    }
}

gen_fn_math_binops!();
