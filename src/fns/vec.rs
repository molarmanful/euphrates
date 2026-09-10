use std::mem;

use crate::{
    fns::EuDef,
    types::EuType,
};

pub const TO_VEC: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?.to_vec()?;
        env.push(EuType::vec(a0));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const WRAP_VEC: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(EuType::vec([a0]));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const ALL_VEC: EuDef = EuDef {
    def: |env| {
        let ts = EuType::Vec(mem::take(&mut env.stack));
        env.push(ts);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const EVAL_VEC: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0 (eval)")?;
        env.push(a0.eval_to_vec(env.scope.clone(), env.ctx)?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const PAIR: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1")?;
        let a0 = env.arg("a0")?;
        env.stack.push(EuType::vec([a0, a1]));
        Ok(())
    },
    sigs: &[],
    doc: "",
};
