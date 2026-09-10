use crate::{
    env::EuEnv,
    fns::EuDef,
    types::EuType,
};

pub const TO_EXPR: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?.to_expr();
        env.push(EuType::res_str(a0.map(EuType::expr)));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const WRAP_EXPR: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(EuType::expr([a0.into()]));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const EVAL: EuDef = EuDef {
    def: |env| env.arg("a0 (eval)")?.for_rec(&mut |f| env.eval_iter(f)),
    sigs: &[],
    doc: "",
};

pub const TAP: EuDef = EuDef {
    def: |env| {
        env.arg("a0 (eval)")?.for_rec(&mut |f| {
            EuEnv::apply(f, &env.stack, env.scope.clone(), env.ctx)?;
            Ok(())
        })
    },
    sigs: &[],
    doc: "",
};

pub const AND_EVAL: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (eval)")?;
        let a0 = env.arg("a0 (cond)")?.into();
        if a0 {
            a1.for_rec(&mut |f| env.eval_iter(f))
        } else {
            Ok(())
        }
    },
    sigs: &[],
    doc: "",
};

pub const OR_EVAL: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (eval)")?;
        let a0 = env.arg("a0 (cond)")?.into();
        if a0 {
            Ok(())
        } else {
            a1.for_rec(&mut |f| env.eval_iter(f))
        }
    },
    sigs: &[],
    doc: "",
};

pub const IF_EVAL: EuDef = EuDef {
    def: |env| {
        let a2 = env.arg("a2 (evalT)")?;
        let a1 = env.arg("a1 (evalF)")?;
        let a0 = env.arg("a0 (cond)")?.into();
        if a0 { a1 } else { a2 }.for_rec(&mut |f| env.eval_iter(f))
    },
    sigs: &[],
    doc: "",
};
