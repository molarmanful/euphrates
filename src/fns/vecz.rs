use crate::{
    fns::{
        EuDef,
        macros::f_2_to_try_1,
    },
    types::EuType,
};

macro_rules! f_env_2_to_try_1 {
    ($f:ident) => {
        |env| {
            let a1 = env.arg("a1 (eval)")?;
            let a0 = env.arg("a0")?;
            env.push(a0.$f(a1, env.scope.clone(), env.ctx)?);
            Ok(())
        }
    };
}

macro_rules! f_env_3_to_try_1 {
    ($f:ident, $a1:literal) => {
        |env| {
            let a2 = env.arg("a2 (eval)")?;
            let a1 = env.arg(concat!("a1", $a1))?;
            let a0 = env.arg("a0")?;
            env.push(a0.$f(a1, a2, env.scope.clone(), env.ctx)?);
            Ok(())
        }
    };
}

pub const GET: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (key)")?;
        let a0 = env.arg("a0")?;
        env.push(EuType::opt(a0.get(&a1)?));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const HAS: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (key)")?;
        let a0 = env.arg("a0")?;
        env.push(EuType::Bool(a0.has(&a1)));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const LEN: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(EuType::ibig(a0.len()));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const SIZE: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(EuType::ibig(a0.size()));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const PUSH_BACK: EuDef = EuDef {
    def: f_2_to_try_1!(push_back),
    sigs: &[],
    doc: "",
};
pub const PUSH_FRONT: EuDef = EuDef {
    def: f_2_to_try_1!(push_front),
    sigs: &[],
    doc: "",
};

pub const INSERT: EuDef = EuDef {
    def: |env| {
        let a2 = env.arg("a2 (index)")?;
        let a1 = env.arg("a1 (item)")?;
        let a0 = env.arg("a0")?;
        env.push(a2.vecz1(|n| a0.insert(n.try_isize()?, a1))?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const APPEND: EuDef = EuDef {
    def: f_2_to_try_1!(append),
    sigs: &[],
    doc: "",
};

pub const POP_BACK: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(a0.pop_back()?.1);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const POP_FRONT: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(a0.pop_front()?.1);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const MOVE_BACK: EuDef = EuDef {
    def: |env| {
        let (t, ts) = env.arg("a0")?.pop_back()?;
        env.push(ts);
        env.push(EuType::opt(t));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const MOVE_FRONT: EuDef = EuDef {
    def: |env| {
        let (t, ts) = env.arg("a0")?.pop_front()?;
        env.push(ts);
        env.push(EuType::opt(t));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const MOVE: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (key)")?;
        let a0 = env.arg("a0")?;
        let (t, ts) = a0.remove(&a1).map(|(t, ts)| (EuType::opt(t), ts))?;
        env.push(ts);
        env.push(t);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const MOVE_INDEX: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (index)")?;
        let a0 = env.arg("a0")?;
        let (t, ts) = a1.vecz1_2(|n| {
            a0.remove_index(n.try_isize()?)
                .map(|(t, ts)| (EuType::opt(t), ts))
        })?;
        env.push(ts);
        env.push(t);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const DELETE: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (key)")?;
        let a0 = env.arg("a0")?;
        env.push(a0.remove(&a1)?.1);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const DELETE_INDEX: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (index)")?;
        let a0 = env.arg("a0")?;
        env.push(a1.vecz1(|n| a0.remove_index(n.try_isize()?).map(|(_, ts)| ts))?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const SWAP_MOVE: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (key)")?;
        let a0 = env.arg("a0")?;
        let (t, ts) = a0.swap_remove(&a1).map(|(t, ts)| (EuType::opt(t), ts))?;
        env.push(ts);
        env.push(t);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const SWAP_MOVE_INDEX: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (index)")?;
        let a0 = env.arg("a0")?;
        let (t, ts) = a1.vecz1_2(|n| {
            a0.swap_remove_index(n.try_isize()?)
                .map(|(t, ts)| (EuType::opt(t), ts))
        })?;
        env.push(ts);
        env.push(t);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const SWAP_DELETE: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (key)")?;
        let a0 = env.arg("a0")?;
        env.push(a0.swap_remove(&a1)?.1);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const SWAP_DELETE_INDEX: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (index)")?;
        let a0 = env.arg("a0")?;
        env.push(a1.vecz1(|n| a0.swap_remove_index(n.try_isize()?).map(|(_, ts)| ts))?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const AT: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (index)")?;
        let a0 = env.arg("a0")?;
        env.push(a1.vecz1(|n| a0.at(n.try_isize()?).map(EuType::opt))?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const TAKE: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (int)")?;
        let a0 = env.arg("a0")?;
        env.push(a1.vecz1(|n| a0.take(n.try_isize()?))?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const DROP: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (int)")?;
        let a0 = env.arg("a0")?;
        env.push(a1.vecz1(|n| a0.drop(n.try_isize()?))?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const CHUNK: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (int)")?;
        let a0 = env.arg("a0")?;
        env.push(a1.vecz1(|n| a0.chunk(n.try_isize()?))?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const WINDOW: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (int)")?;
        let a0 = env.arg("a0")?;
        env.push(a1.vecz1(|n| a0.window(n.try_usize()?))?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const DIVVY: EuDef = EuDef {
    def: |env| {
        let a2 = env.arg("a2 (offset)")?;
        let a1 = env.arg("a1 (size)")?;
        let a0 = env.arg("a0")?;
        env.push(a1.vecz2(a2, |n, o| a0.divvy(n.try_usize()?, o.try_isize()?))?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const SORT: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(a0.sort()?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const FLAT: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(a0.flatten()?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const FLAT_REC: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(a0.flatten_rec()?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const ENUM: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(a0.enumerate());
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const PAIRS: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?;
        env.push(a0.pairs());
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const MULTI_ZIP: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?.to_vec()?;
        env.push(EuType::seq(EuType::multi_zip(a0)));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const MULTI_CPROD: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?.to_vec()?;
        env.push(EuType::seq(EuType::multi_cartesian_product(a0)));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const MAP: EuDef = EuDef {
    def: f_env_2_to_try_1!(map_env),
    sigs: &[],
    doc: "",
};

pub const MAP_ATOM: EuDef = EuDef {
    def: f_env_2_to_try_1!(map_atom_env),
    sigs: &[],
    doc: "",
};

pub const FLAT_MAP: EuDef = EuDef {
    def: f_env_2_to_try_1!(flat_map_env),
    sigs: &[],
    doc: "",
};

pub const FILTER: EuDef = EuDef {
    def: f_env_2_to_try_1!(filter_env),
    sigs: &[],
    doc: "",
};

pub const TAKE_WHILE: EuDef = EuDef {
    def: f_env_2_to_try_1!(take_while_env),
    sigs: &[],
    doc: "",
};

pub const DROP_WHILE: EuDef = EuDef {
    def: f_env_2_to_try_1!(drop_while_env),
    sigs: &[],
    doc: "",
};

pub const FOLD1: EuDef = EuDef {
    def: f_env_2_to_try_1!(fold1_env),
    sigs: &[],
    doc: "",
};

pub const SORT_BY: EuDef = EuDef {
    def: f_env_2_to_try_1!(sort_by_env),
    sigs: &[],
    doc: "",
};

pub const SORT_BY_KEY: EuDef = EuDef {
    def: f_env_2_to_try_1!(sort_by_key_env),
    sigs: &[],
    doc: "",
};

pub const FIND: EuDef = EuDef {
    def: f_env_2_to_try_1!(find_env),
    sigs: &[],
    doc: "",
};

pub const ANY: EuDef = EuDef {
    def: f_env_2_to_try_1!(any_env),
    sigs: &[],
    doc: "",
};

pub const ALL: EuDef = EuDef {
    def: f_env_2_to_try_1!(all_env),
    sigs: &[],
    doc: "",
};

pub const ZIP: EuDef = EuDef {
    def: f_env_3_to_try_1!(zip_env, ""),
    sigs: &[],
    doc: "",
};

pub const FOLD: EuDef = EuDef {
    def: f_env_3_to_try_1!(fold_env, " (acc)"),
    sigs: &[],
    doc: "",
};

pub const SCAN: EuDef = EuDef {
    def: f_env_3_to_try_1!(scan_env, " (acc)"),
    sigs: &[],
    doc: "",
};

pub const ZIP_ATOM: EuDef = EuDef {
    def: f_env_3_to_try_1!(zip_atom_env, ""),
    sigs: &[],
    doc: "",
};

pub const SEP: EuDef = EuDef {
    def: f_2_to_try_1!(sep),
    sigs: &[],
    doc: "",
};
