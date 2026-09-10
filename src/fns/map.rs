use std::mem;

use crate::{
    fns::EuDef,
    types::EuType,
};

pub const TO_MAP: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?.to_map()?;
        env.push(EuType::Map(a0));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const WRAP_MAP: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(EuType::map_([(EuType::I64(0), a0)]));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const ALL_MAP: EuDef = EuDef {
    def: |env| {
        let kvs = EuType::Vec(mem::take(&mut env.stack)).to_map()?;
        env.push(EuType::Map(kvs));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const EVAL_MAP: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0 (eval)")?;
        env.push(a0.eval_to_map(env.scope.clone(), env.ctx)?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};
