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
        macros::f_2_to_try_1,
    },
    types::EuType,
};

macro_rules! f64_f_1_to_1 {
    ($f:ident) => {
        |env| {
            let a0 = env.arg("a0")?;
            env.push(a0.vecz1(|t| Ok(EuType::f64(t.try_f64()?.$f())))?);
            Ok(())
        }
    };
}

macro_rules! f64_f_2_to_1 {
    ($f:ident) => {
        |env| {
            let a1 = env.arg("a1")?;
            let a0 = env.arg("a0")?;
            env.push(a0.vecz2(a1, |a, b| Ok(EuType::f64(a.try_f64()?.$f(b.try_f64()?))))?);
            Ok(())
        }
    };
}

macro_rules! f64_f_1_to_2 {
    ($f:ident) => {
        |env| {
            let a0 = env.arg("a0")?;
            let (b0, b1) = a0.vecz1_2(|t| {
                let (b0, b1) = t.try_f64()?.$f();
                Ok((EuType::f64(b0), EuType::f64(b1)))
            })?;
            env.push(b0);
            env.push(b1);
            Ok(())
        }
    };
}

pub const MIN_I32: EuDef = EuDef {
    def: |env| {
        env.push(EuType::I32(i32::MIN));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const MAX_I32: EuDef = EuDef {
    def: |env| {
        env.push(EuType::I32(i32::MAX));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const MIN_I64: EuDef = EuDef {
    def: |env| {
        env.push(EuType::I64(i64::MIN));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const MAX_I64: EuDef = EuDef {
    def: |env| {
        env.push(EuType::I64(i64::MAX));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const MIN_F64: EuDef = EuDef {
    def: |env| {
        env.push(EuType::F64(OrderedFloat::min_value()));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const MAX_F64: EuDef = EuDef {
    def: |env| {
        env.push(EuType::F64(OrderedFloat::max_value()));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const INF: EuDef = EuDef {
    def: |env| {
        env.push(EuType::F64(OrderedFloat::infinity()));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const NAN: EuDef = EuDef {
    def: |env| {
        env.push(EuType::F64(OrderedFloat::nan()));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const PI: EuDef = EuDef {
    def: |env| {
        env.push(EuType::F64(OrderedFloat::PI()));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const E: EuDef = EuDef {
    def: |env| {
        env.push(EuType::F64(OrderedFloat::E()));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const EPSILON: EuDef = EuDef {
    def: |env| {
        env.push(EuType::F64(OrderedFloat::epsilon()));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const TO_I32: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(EuType::opt(a0.to_i32().map(EuType::i32)));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const TO_I64: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(EuType::opt(a0.to_i64().map(EuType::i64)));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const TO_F64: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(EuType::opt(a0.to_f64().map(EuType::f64)));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const TO_IBIG: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(EuType::opt(a0.to_ibig().map(EuType::ibig)));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const NEG: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push((-a0)?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const ADD: EuDef = EuDef {
    def: f_2_to_try_1!(add),
    sigs: &[],
    doc: "",
};

pub const SUB: EuDef = EuDef {
    def: f_2_to_try_1!(sub),
    sigs: &[],
    doc: "",
};

pub const MUL: EuDef = EuDef {
    def: f_2_to_try_1!(mul),
    sigs: &[],
    doc: "",
};

pub const DIV: EuDef = EuDef {
    def: f_2_to_try_1!(div),
    sigs: &[],
    doc: "",
};

pub const REM: EuDef = EuDef {
    def: f_2_to_try_1!(rem),
    sigs: &[],
    doc: "",
};

pub const POW: EuDef = EuDef {
    def: f_2_to_try_1!(pow),
    sigs: &[],
    doc: "",
};

pub const DIV_REM: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1")?;
        let a0 = env.arg("a0")?;
        env.push((a0.clone() / a1.clone())?);
        env.push((a0 % a1)?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const EXP: EuDef = EuDef {
    def: f64_f_1_to_1!(exp),
    sigs: &[],
    doc: "",
};

pub const EXP_M1: EuDef = EuDef {
    def: f64_f_1_to_1!(exp_m1),
    sigs: &[],
    doc: "",
};

pub const SQRT: EuDef = EuDef {
    def: f64_f_1_to_1!(sqrt),
    sigs: &[],
    doc: "",
};

pub const CBRT: EuDef = EuDef {
    def: f64_f_1_to_1!(cbrt),
    sigs: &[],
    doc: "",
};

pub const HYPOT: EuDef = EuDef {
    def: f64_f_2_to_1!(hypot),
    sigs: &[],
    doc: "",
};

pub const LOG: EuDef = EuDef {
    def: f64_f_2_to_1!(log),
    sigs: &[],
    doc: "",
};

pub const LN: EuDef = EuDef {
    def: f64_f_1_to_1!(ln),
    sigs: &[],
    doc: "",
};

pub const LN_1P: EuDef = EuDef {
    def: f64_f_1_to_1!(ln_1p),
    sigs: &[],
    doc: "",
};

pub const SIN_COS: EuDef = EuDef {
    def: f64_f_1_to_2!(sin_cos),
    sigs: &[],
    doc: "",
};

pub const SIN: EuDef = EuDef {
    def: f64_f_1_to_1!(sin),
    sigs: &[],
    doc: "",
};

pub const COS: EuDef = EuDef {
    def: f64_f_1_to_1!(cos),
    sigs: &[],
    doc: "",
};

pub const TAN: EuDef = EuDef {
    def: f64_f_1_to_1!(tan),
    sigs: &[],
    doc: "",
};

pub const ASIN: EuDef = EuDef {
    def: f64_f_1_to_1!(asin),
    sigs: &[],
    doc: "",
};

pub const ACOS: EuDef = EuDef {
    def: f64_f_1_to_1!(acos),
    sigs: &[],
    doc: "",
};

pub const ATAN: EuDef = EuDef {
    def: f64_f_1_to_1!(atan),
    sigs: &[],
    doc: "",
};

pub const ATAN2: EuDef = EuDef {
    def: f64_f_2_to_1!(atan2),
    sigs: &[],
    doc: "",
};

pub const SINH: EuDef = EuDef {
    def: f64_f_1_to_1!(sinh),
    sigs: &[],
    doc: "",
};

pub const COSH: EuDef = EuDef {
    def: f64_f_1_to_1!(cosh),
    sigs: &[],
    doc: "",
};

pub const TANH: EuDef = EuDef {
    def: f64_f_1_to_1!(tanh),
    sigs: &[],
    doc: "",
};

pub const ASINH: EuDef = EuDef {
    def: f64_f_1_to_1!(asinh),
    sigs: &[],
    doc: "",
};

pub const ACOSH: EuDef = EuDef {
    def: f64_f_1_to_1!(acosh),
    sigs: &[],
    doc: "",
};

pub const ATANH: EuDef = EuDef {
    def: f64_f_1_to_1!(atanh),
    sigs: &[],
    doc: "",
};
