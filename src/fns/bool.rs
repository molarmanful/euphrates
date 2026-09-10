use crate::{
    fns::EuDef,
    types::EuType,
};

pub const TRUE: EuDef = EuDef {
    def: |env| {
        env.push(EuType::Bool(true));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const FALSE: EuDef = EuDef {
    def: |env| {
        env.push(EuType::Bool(false));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const TO_BOOL: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(EuType::Bool(a0.into()));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const NOT: EuDef = EuDef {
    def: |env| {
        let a0: bool = env.arg("a0")?.into();
        env.push(EuType::Bool(!a0));
        Ok(())
    },
    sigs: &[],
    doc: "",
};
