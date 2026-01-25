use std::iter;

use crate::{
    fns::EuDef,
    types::EuType,
};

pub const SEQ_N0: EuDef = |env| {
    env.push(EuType::seq((0..).map(EuType::ibig).map(Ok)));
    Ok(())
};

pub const TO_SEQ: EuDef = |env| {
    let a0 = env.pop()?.to_seq();
    env.push(EuType::Seq(a0));
    Ok(())
};

pub const WRAP_SEQ: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::seq(iter::once(Ok(a0))));
    Ok(())
};

pub const UNFOLD: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.unfold_env(a1, env.scope.clone())?);
    Ok(())
};

pub const REPEAT: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::seq(a0.repeat()));
    Ok(())
};

pub const REPEAT_N: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a1.vecz1(|n| a0.repeat_n(n.try_usize()?).map(EuType::Vec))?);
    Ok(())
};

pub const CYCLE: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::seq(a0.cycle()));
    Ok(())
};
