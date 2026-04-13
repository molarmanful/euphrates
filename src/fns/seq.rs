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
    let a0 = env.arg("a0")?.to_seq();
    env.push(EuType::Seq(a0));
    Ok(())
};

pub const WRAP_SEQ: EuDef = |env| {
    let a0 = env.arg("a0")?;
    env.push(EuType::seq(iter::once(Ok(a0))));
    Ok(())
};

pub const UNFOLD: EuDef = |env| {
    let a1 = env.arg("a1 (eval)")?;
    let a0 = env.arg("a0")?;
    env.push(a0.unfold_env(a1, env.scope.clone(), env.opts)?);
    Ok(())
};

pub const REPEAT: EuDef = |env| {
    let a0 = env.arg("a0")?;
    env.push(EuType::seq(a0.repeat()));
    Ok(())
};

pub const REPEAT_N: EuDef = |env| {
    let a1 = env.arg("a1 (num)")?;
    let a0 = env.arg("a0")?;
    env.push(a1.vecz1(|n| a0.repeat_n(n.try_usize()?).map(EuType::Vec))?);
    Ok(())
};

pub const CYCLE: EuDef = |env| {
    let a0 = env.arg("a0")?;
    env.push(EuType::seq(a0.cycle()));
    Ok(())
};
