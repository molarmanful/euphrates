use std::mem;

use crate::{
    fns::EuDef,
    types::EuType,
};

pub const TO_SET: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?.to_set()?;
        env.push(EuType::Set(a0));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const WRAP_SET: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(EuType::set([a0]));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const ALL_SET: EuDef = EuDef {
    def: |env| {
        let ts = EuType::Vec(mem::take(&mut env.stack)).to_set()?;
        env.push(EuType::Set(ts));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const EVAL_SET: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0 (eval)")?;
        env.push(a0.eval_to_set(env.scope.clone(), env.ctx)?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};
