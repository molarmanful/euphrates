use crate::{
    env::EuEnv,
    fns::EuDef,
    types::EuType,
};

pub const TO_EXPR: EuDef = |env| {
    let a0 = env.arg("a0")?.to_expr();
    env.push(EuType::res_str(a0.map(EuType::expr)));
    Ok(())
};

pub const WRAP_EXPR: EuDef = |env| {
    let a0 = env.arg("a0")?;
    env.push(EuType::expr([a0.into()]));
    Ok(())
};

pub const EVAL: EuDef = |env| env.arg("a0 (eval)")?.for_rec(&mut |f| env.eval_iter(f));

pub const TAP: EuDef = |env| {
    env.arg("a0 (eval)")?.for_rec(&mut |f| {
        EuEnv::apply(f, &env.stack, env.scope.clone(), env.ctx)?;
        Ok(())
    })
};

pub const AND_EVAL: EuDef = |env| {
    let a1 = env.arg("a1 (eval)")?;
    let a0 = env.arg("a0 (cond)")?.into();
    if a0 {
        a1.for_rec(&mut |f| env.eval_iter(f))
    } else {
        Ok(())
    }
};

pub const OR_EVAL: EuDef = |env| {
    let a1 = env.arg("a1 (eval)")?;
    let a0 = env.arg("a0 (cond)")?.into();
    if a0 {
        Ok(())
    } else {
        a1.for_rec(&mut |f| env.eval_iter(f))
    }
};

pub const IF_EVAL: EuDef = |env| {
    let a2 = env.arg("a2 (evalT)")?;
    let a1 = env.arg("a1 (evalF)")?;
    let a0 = env.arg("a0 (cond)")?.into();
    if a0 { a1 } else { a2 }.for_rec(&mut |f| env.eval_iter(f))
};
