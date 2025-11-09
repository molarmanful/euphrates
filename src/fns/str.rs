use super::EuDef;
use crate::types::EuType;

pub const TO_STR: EuDef = |env| {
    let a0 = match env.pop()? {
        t @ EuType::Str(_) => t,
        t => EuType::str(t.to_string()),
    };
    env.push(a0);
    Ok(())
};
