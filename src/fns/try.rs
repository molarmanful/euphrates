use crate::{
    env::EuEnv,
    fns::EuDef,
    types::EuType,
};

pub const NONE: EuDef = EuDef {
    def: |env| {
        env.push(EuType::Opt(None));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const SOME: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(EuType::opt(Some(a0)));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const OK: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(EuType::res(Ok(a0)));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const ERR: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(EuType::res(Err(a0)));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const EVAL_RES: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0 (eval)")?.to_expr()?;
        env.push(EuType::res_str(EuEnv::apply_n_1(
            a0,
            &env.stack,
            env.scope.clone(),
            env.ctx,
        )));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const COALESCE: EuDef = EuDef {
    def: |env| {
        let a0 = match env.arg("a0")? {
            t @ (EuType::Opt(None) | EuType::Res(Err(_))) => {
                env.clear_queue();
                t
            }
            EuType::Opt(Some(t)) | EuType::Res(Ok(t)) => *t,
            t => t,
        };
        env.push(a0);
        Ok(())
    },
    sigs: &[],
    doc: "",
};
