use std::rc::Rc;

use anyhow::{
    Context,
    anyhow,
};
use ecow::EcoVec;
use ordermap::OrderMap;

use crate::{
    env::EuEnv,
    types::{
        EuBind,
        EuRes,
        EuType,
    },
};

pub struct EuBindDef {
    pub bind: for<'eu> fn(&mut EuEnv<'eu>, &EcoVec<EuBind<'eu>>, EuType<'eu>) -> EuRes<()>,
    pub free: for<'eu> fn(EcoVec<EuBind<'eu>>) -> Option<EuType<'eu>>,
}

pub const BIND: phf::Map<&str, EuBindDef> = phf::phf_map! {
    "Vecz" => VECZ,
    "Map" => MAP,
    "Some" => SOME,
    "None" => NONE,
    "Ok" => OK,
    "Err" => ERR,
};

pub const VECZ: EuBindDef = EuBindDef {
    bind: |env, bs, t| {
        let mut it = t.to_seq();
        for b in bs {
            let t = it.next().with_context(|| format!("missing `{b:?}`"))??;
            env.bind_type(b, t)?;
        }
        Ok(())
    },
    free: |bs| {
        bs.into_iter()
            .map(EuBind::to_free)
            .try_collect()
            .map(EuType::Vec)
    },
};

pub const MAP: EuBindDef = EuBindDef {
    bind: |env, bs, t| {
        fn inner<'eu>(
            env: &mut EuEnv<'eu>,
            b: &EuBind<'eu>,
            kvs: &Rc<OrderMap<EuType<'eu>, EuType<'eu>>>,
        ) -> EuRes<()> {
            match b {
                EuBind::Word(w) => {
                    let k = EuType::Str(w.clone());
                    env.scope.insert(w.clone(), get_key(kvs, &k)?.clone());
                }
                EuBind::Bind(b0, b1) => {
                    if let Some(k) = b0.clone().to_free() {
                        env.bind_type(b1, get_key(kvs, &k)?.clone())?;
                    } else {
                        worst_bind(env, b1, kvs)?;
                    }
                }
                EuBind::Union(bs) => {
                    for b in bs {
                        if inner(env, b, kvs).is_ok() {
                            return Ok(());
                        }
                    }
                }
                _ if let Some(k) = b.clone().to_free() => {
                    get_key(kvs, &k)?;
                }
                _ => worst_bind(env, b, kvs)?,
            }
            Ok(())
        }

        fn get_key<'eu, 'kvs>(
            kvs: &'kvs Rc<OrderMap<EuType<'eu>, EuType<'eu>>>,
            k: &EuType<'eu>,
        ) -> EuRes<&'kvs EuType<'eu>> {
            Ok(kvs.get(k).with_context(|| format!("missing key `{k:?}`"))?)
        }

        fn worst_bind<'eu>(
            env: &mut EuEnv<'eu>,
            b: &EuBind<'eu>,
            kvs: &Rc<OrderMap<EuType<'eu>, EuType<'eu>>>,
        ) -> EuRes<()> {
            for k in kvs.keys().cloned() {
                if env.bind_type(b, k).is_ok() {
                    return Ok(());
                }
            }
            Err(anyhow!("failed to bind {b:?}").into())
        }

        let kvs = t.to_map()?;
        for b in bs {
            inner(env, b, &kvs)?;
        }
        Ok(())
    },

    free: VECZ.free,
};

pub const SOME: EuBindDef = EuBindDef {
    bind: |env, bs, t| {
        if let EuType::Opt(Some(t)) = t {
            let b0 = bs.into_iter().next().context("missing arg `b0`")?;
            env.bind_type(b0, *t)
        } else {
            Err(anyhow!("expected Some, got `{t:?}`").into())
        }
    },
    free: |bs| {
        bs.into_iter()
            .next()
            .and_then(EuBind::to_free)
            .map(Some)
            .map(EuType::opt)
    },
};

pub const NONE: EuBindDef = EuBindDef {
    bind: |_, _, t| {
        matches!(t, EuType::Opt(None)).ok_or_else(|| anyhow!("expected None, got `{t:?}`").into())
    },
    free: |_| Some(EuType::opt(None)),
};

pub const OK: EuBindDef = EuBindDef {
    bind: |env, bs, t| {
        if let EuType::Res(Ok(t)) = t {
            let b0 = bs.into_iter().next().context("missing arg `b0`")?;
            env.bind_type(b0, *t)
        } else {
            Err(anyhow!("expected Ok, got `{t:?}`").into())
        }
    },
    free: |bs| {
        bs.into_iter()
            .next()
            .and_then(EuBind::to_free)
            .map(Ok)
            .map(EuType::res)
    },
};

pub const ERR: EuBindDef = EuBindDef {
    bind: |env, bs, t| {
        if let EuType::Res(Err(t)) = t {
            let b0 = bs.into_iter().next().context("missing arg `b0`")?;
            env.bind_type(b0, *t)
        } else {
            Err(anyhow!("expected Err, got `{t:?}`").into())
        }
    },
    free: |bs| {
        bs.into_iter()
            .next()
            .and_then(EuBind::to_free)
            .map(Err)
            .map(EuType::res)
    },
};
