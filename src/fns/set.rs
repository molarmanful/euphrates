use std::mem;

use crate::{
    fns::EuDef,
    types::EuType,
};

pub const TO_SET: EuDef = |env| {
    let a0 = env.pop()?.to_set()?;
    env.push(EuType::Set(a0));
    Ok(())
};

pub const WRAP_SET: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::set([a0]));
    Ok(())
};

pub const ALL_SET: EuDef = |env| {
    let ts = EuType::Vec(mem::take(&mut env.stack)).to_set()?;
    env.push(EuType::Set(ts));
    Ok(())
};

pub const EVAL_SET: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.eval_to_set(env.scope.clone())?);
    Ok(())
};
