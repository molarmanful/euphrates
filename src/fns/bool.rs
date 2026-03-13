use crate::{
    fns::EuDef,
    types::EuType,
};

pub const TRUE: EuDef = |env| {
    env.push(EuType::Bool(true));
    Ok(())
};

pub const FALSE: EuDef = |env| {
    env.push(EuType::Bool(false));
    Ok(())
};

pub const TO_BOOL: EuDef = |env| {
    let a0 = env.arg("a0")?;
    env.push(EuType::Bool(a0.into()));
    Ok(())
};

pub const NOT: EuDef = |env| {
    let a0: bool = env.arg("a0")?.into();
    env.push(EuType::Bool(!a0));
    Ok(())
};
