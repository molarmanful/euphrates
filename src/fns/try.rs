use super::EuDef;
use crate::{
    env::EuEnv,
    types::EuType,
};

pub const NONE: EuDef = |env| {
    env.push(EuType::Opt(None));
    Ok(())
};

pub const SOME: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::opt(Some(a0)));
    Ok(())
};

pub const OK: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::res(Ok(a0)));
    Ok(())
};

pub const ERR: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::res(Err(a0)));
    Ok(())
};

pub const EVAL_RES: EuDef = |env| {
    let a0 = env.pop()?.to_expr()?;
    env.push(EuType::res_str(EuEnv::apply_n_1(
        a0,
        &env.stack,
        env.scope.clone(),
    )));
    Ok(())
};

pub const TRY: EuDef = |env| {
    let a0 = match env.pop()? {
        t @ (EuType::Opt(None) | EuType::Res(Err(_))) => {
            env.clear_queue();
            t
        }
        EuType::Opt(Some(t)) | EuType::Res(Ok(t)) => *t,
        t => t,
    };
    env.push(a0);
    Ok(())
};
