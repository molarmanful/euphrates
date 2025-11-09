use super::EuDef;
use crate::{
    env::EuEnv,
    types::EuType,
};

pub const TO_EXPR: EuDef = |env| {
    let a0 = env.pop()?.to_expr();
    env.push(EuType::res_str(a0.map(EuType::expr)));
    Ok(())
};

pub const WRAP_EXPR: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::expr([a0.into()]));
    Ok(())
};

pub const EVAL: EuDef = |env| env.pop()?.for_rec(&mut |f| env.eval_iter(f));

pub const TAP: EuDef = |env| {
    env.pop()?.for_rec(&mut |f| {
        EuEnv::apply(f, &env.stack, env.scope.clone())?;
        Ok(())
    })
};

pub const AND_EVAL: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap().into();
    if a0 {
        a1.for_rec(&mut |f| env.eval_iter(f))
    } else {
        Ok(())
    }
};

pub const OR_EVAL: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap().into();
    if a0 {
        Ok(())
    } else {
        a1.for_rec(&mut |f| env.eval_iter(f))
    }
};

pub const IF_EVAL: EuDef = |env| {
    env.check_nargs(3)?;
    let a2 = env.stack.pop().unwrap();
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap().into();
    if a0 { a1 } else { a2 }.for_rec(&mut |f| env.eval_iter(f))
};

pub const BIND_ARGS: EuDef = |env| {
    let a0 = env.pop()?;
    env.bind_args(a0.into())
};
