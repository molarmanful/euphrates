use crate::{
    fns::EuDef,
    types::EuType,
};

pub const TO_STR: EuDef = EuDef {
    def: |env| {
        let a0 = match env.arg("a0")? {
            t @ EuType::Str(_) => t,
            t => EuType::str(t.to_string()),
        };
        env.push(a0);
        Ok(())
    },
    sigs: &[],
    doc: "",
};
