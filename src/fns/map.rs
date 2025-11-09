use std::mem;

use super::EuDef;
use crate::types::EuType;

pub const TO_MAP: EuDef = |env| {
    let a0 = env.pop()?.to_map()?;
    env.push(EuType::Map(a0));
    Ok(())
};

pub const WRAP_MAP: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::map_([(EuType::I64(0), a0)]));
    Ok(())
};

pub const ALL_MAP: EuDef = |env| {
    let kvs = EuType::Vec(mem::take(&mut env.stack)).to_map()?;
    env.push(EuType::Map(kvs));
    Ok(())
};

pub const EVAL_MAP: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.eval_to_map(env.scope.clone())?);
    Ok(())
};
