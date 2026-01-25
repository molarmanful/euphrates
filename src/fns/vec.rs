use std::mem;

use crate::{
    fns::EuDef,
    types::EuType,
};

pub const TO_VEC: EuDef = |env| {
    let a0 = env.pop()?.to_vec()?;
    env.push(EuType::vec(a0));
    Ok(())
};

pub const WRAP_VEC: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::vec([a0]));
    Ok(())
};

pub const ALL_VEC: EuDef = |env| {
    let ts = EuType::Vec(mem::take(&mut env.stack));
    env.push(ts);
    Ok(())
};

pub const EVAL_VEC: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.eval_to_vec(env.scope.clone())?);
    Ok(())
};

pub const PAIR: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.stack.push(EuType::vec([a0, a1]));
    Ok(())
};
