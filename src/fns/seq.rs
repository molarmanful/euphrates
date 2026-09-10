use std::iter;

use crate::{
    fns::EuDef,
    types::EuType,
};

pub const SEQ_N0: EuDef = EuDef {
    def: |env| {
        env.push(EuType::seq((0..).map(EuType::ibig).map(Ok)));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const TO_SEQ: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?.to_seq();
        env.push(EuType::Seq(a0));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const WRAP_SEQ: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(EuType::seq(iter::once(Ok(a0))));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const UNFOLD: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (eval)")?;
        let a0 = env.arg("a0")?;
        env.push(a0.unfold_env(a1, env.scope.clone(), env.ctx)?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const REPEAT: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(EuType::seq(a0.repeat()));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const REPEAT_N: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (num)")?;
        let a0 = env.arg("a0")?;
        env.push(a1.vecz1(|n| a0.repeat_n(n.try_usize()?).map(EuType::Vec))?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const CYCLE: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(EuType::seq(a0.cycle()));
        Ok(())
    },
    sigs: &[],
    doc: "",
};
