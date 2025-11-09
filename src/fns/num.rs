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

use super::EuDef;
use crate::types::EuType;

#[crabtime::function]
fn gen_fn_int_consts() {
    let types = ["I32", "I64"];
    let consts = ["MIN", "MAX"];
    for t in types {
        let n = t.to_lowercase();
        for c in consts {
            crabtime::output! {
                pub const {{c}}_{{t}}: EuDef = |env| {
                    env.push(EuType::{{t}}({{n}}::{{c}}));
                    Ok(())
                };
            };
        }
    }
}

gen_fn_int_consts!();

#[crabtime::function]
fn gen_fn_float_consts() {
    let types = ["F32", "F64"];
    let consts = [("MIN", "min_value"), ("MAX", "max_value")];
    for t in types {
        for (c, f) in consts {
            crabtime::output! {
                pub const {{c}}_{{t}}: EuDef = |env| {
                    env.push(EuType::{{t}}(OrderedFloat::{{f}}()));
                    Ok(())
                };
            };
        }
    }
}

gen_fn_float_consts!();

pub const INF: EuDef = |env| {
    env.push(EuType::F64(OrderedFloat::infinity()));
    Ok(())
};

pub const INF32: EuDef = |env| {
    env.push(EuType::F32(OrderedFloat::infinity()));
    Ok(())
};

pub const NAN: EuDef = |env| {
    env.push(EuType::F64(OrderedFloat::nan()));
    Ok(())
};

pub const NAN32: EuDef = |env| {
    env.push(EuType::F32(OrderedFloat::nan()));
    Ok(())
};

#[crabtime::function]
fn gen_def_to_num() {
    let types = ["I32", "I64", "F32", "F64", "IBig"];
    for &t in &types {
        let n = t.to_lowercase();
        let n_up = t.to_uppercase();
        crabtime::output! {
            pub const TO_{{n_up}}: EuDef = |env| {
                let a0 = env.pop()?;
                env.push(EuType::opt(a0.to_{{n}}().map(EuType::{{n}})));
                Ok(())
            };
        }
    }
}

gen_def_to_num!();
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
