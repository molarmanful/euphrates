use std::mem;

use super::EuDef;
use crate::{
    env::EuEnv,
    types::EuType,
};

pub const STACK: EuDef = |env| {
    env.push(EuType::Vec(env.stack.clone()));
    Ok(())
};

pub const DUP: EuDef = |env| {
    env.push(env.last()?.clone());
    Ok(())
};

pub const DUPD: EuDef = |env| {
    env.check_nargs(2)?;
    env.stack.insert(
        env.iflip(1).unwrap(),
        env.stack[env.iflip(1).unwrap()].clone(),
    );
    Ok(())
};

pub const OVER: EuDef = |env| {
    env.check_nargs(2)?;
    env.push(env.stack[env.iflip(1).unwrap()].clone());
    Ok(())
};

pub const DDUP: EuDef = |env| {
    env.check_nargs(2)?;
    env.push(env.stack[env.iflip(1).unwrap()].clone());
    env.push(env.stack[env.iflip(1).unwrap()].clone());
    Ok(())
};

pub const EDUP: EuDef = |env| {
    env.check_nargs(3)?;
    for _ in 0..3 {
        env.push(env.stack[env.iflip(2).unwrap()].clone());
    }
    Ok(())
};

pub const PICK: EuDef = |env| {
    let a0 = env.pop()?.try_isize()?;
    env.push(env.stack[env.iflip(a0)?].clone());
    Ok(())
};

pub const POP: EuDef = |env| {
    env.pop()?;
    Ok(())
};

pub const CLR: EuDef = |env| {
    env.stack.clear();
    Ok(())
};

pub const NIP: EuDef = |env| {
    env.check_nargs(2)?;
    env.stack.remove(env.iflip(1).unwrap());
    Ok(())
};

pub const PPOP: EuDef = |env| {
    env.check_nargs(2)?;
    env.stack.truncate(env.iflip(1).unwrap());
    Ok(())
};

pub const QPOP: EuDef = |env| {
    env.check_nargs(3)?;
    env.stack.truncate(env.iflip(2).unwrap());
    Ok(())
};

pub const NIX: EuDef = |env| {
    let a0 = env.pop()?.try_isize()?;
    env.stack.remove(env.iflip(a0)?);
    Ok(())
};

pub const SWAP: EuDef = |env| {
    env.check_nargs(2)?;
    let a = env.iflip(0).unwrap();
    env.stack.make_mut().swap(a, a - 1);
    Ok(())
};

pub const REV: EuDef = |env| {
    env.stack.make_mut().reverse();
    Ok(())
};

pub const SWAPD: EuDef = |env| {
    env.check_nargs(3)?;
    let a = env.iflip(1).unwrap();
    env.stack.make_mut().swap(a, a - 1);
    Ok(())
};

pub const TUCK: EuDef = |env| {
    env.check_nargs(2)?;
    env.stack
        .insert(env.iflip(1).unwrap(), env.stack.last().unwrap().clone());
    Ok(())
};

pub const TRADE: EuDef = |env| {
    let a0 = env.pop()?.try_isize()?;
    let i = env.iflip(a0)?;
    let j = env.iflip(0).unwrap();
    env.stack.make_mut().swap(i, j);
    Ok(())
};

pub const ROT: EuDef = |env| {
    env.check_nargs(3)?;
    let a0 = env.stack.remove(env.iflip(2).unwrap());
    env.push(a0);
    Ok(())
};

pub const UNROT: EuDef = |env| {
    env.check_nargs(3)?;
    let a0 = env.stack.pop().unwrap();
    env.stack.insert(env.iflip(1).unwrap(), a0);
    Ok(())
};

pub const ROLL: EuDef = |env| {
    let a0 = env.pop()?.try_isize()?;
    let t = env.stack.remove(env.iflip(a0)?);
    env.push(t);
    Ok(())
};

pub const UNROLL: EuDef = |env| {
    env.check_nargs(2)?;
    let a0 = env.stack.pop().unwrap().try_isize()?;
    let i = env.iflip(a0)?;
    let t = env.stack.pop().unwrap();
    env.stack.insert(i, t);
    Ok(())
};

pub const WRAP: EuDef = |env| {
    let a0 = EuType::Vec(mem::take(&mut env.stack));
    env.stack.push(a0);
    Ok(())
};

pub const UNWRAP: EuDef = |env| {
    let a0 = env.pop()?.to_vec()?;
    env.stack.extend(a0);
    Ok(())
};

pub const USURP: EuDef = |env| {
    env.stack = env.pop()?.to_vec()?;
    Ok(())
};

pub const SUB_STACK: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap().to_expr()?;
    let a0 = env.stack.pop().unwrap().to_vec()?;
    env.push(EuType::Vec(EuEnv::apply(a1, &a0, env.scope.clone())?.stack));
    Ok(())
};

pub const DIP: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap().to_expr()?;
    let a0 = env.stack.pop().unwrap();
    env.stack = EuEnv::apply(a1, &env.stack, env.scope.clone())?.stack;
    env.push(a0);
    Ok(())
};
