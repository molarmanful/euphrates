use crate::{
    fns::{
        EuDef,
        macros::f_2_try_1,
    },
    types::EuType,
};

#[crabtime::function]
fn f_env_2_to_try_1(name: String) {
    let f = format!("{}_env", name.to_lowercase());
    crabtime::output! {
        pub const {{name}}: EuDef = |env| {
            let a1 = env.arg("a1 (eval)")?;
            let a0 = env.arg("a0")?;
            env.push(a0.{{f}}(a1, env.scope.clone(), env.ctx)?);
            Ok(())
        };
    }
}

#[crabtime::function]
fn f_env_3_to_try_1(name: String, a1: String) {
    let f = format!("{}_env", name.to_lowercase());
    let a1 = format!(r#""a1{a1}""#);
    crabtime::output! {
        pub const {{name}}: EuDef = |env| {
            let a2 = env.arg("a2 (eval)")?;
            let a1 = env.arg({{a1}})?;
            let a0 = env.arg("a0")?;
            env.push(a0.{{f}}(a1, a2, env.scope.clone(), env.ctx)?);
            Ok(())
        };
    }
}

pub const GET: EuDef = |env| {
    let a1 = env.arg("a1 (key)")?;
    let a0 = env.arg("a0")?;
    env.push(EuType::opt(a0.get(&a1)?));
    Ok(())
};

pub const HAS: EuDef = |env| {
    let a1 = env.arg("a1 (key)")?;
    let a0 = env.arg("a0")?;
    env.push(EuType::Bool(a0.has(&a1)));
    Ok(())
};

f_2_try_1!(PUSH_BACK);
f_2_try_1!(PUSH_FRONT);

pub const INSERT: EuDef = |env| {
    let a2 = env.arg("a2 (index)")?;
    let a1 = env.arg("a1 (item)")?;
    let a0 = env.arg("a0")?;
    env.push(a2.vecz1(|n| a0.insert(n.try_isize()?, a1))?);
    Ok(())
};

f_2_try_1!(APPEND);

pub const POP_BACK: EuDef = |env| {
    let a0 = env.arg("a0")?;
    env.push(a0.pop_back()?.1);
    Ok(())
};

pub const POP_FRONT: EuDef = |env| {
    let a0 = env.arg("a0")?;
    env.push(a0.pop_front()?.1);
    Ok(())
};

pub const REMOVE_INDEX: EuDef = |env| {
    let a1 = env.arg("a1 (index)")?;
    let a0 = env.arg("a0")?;
    env.push(a1.vecz1(|n| a0.remove(n.try_isize()?).map(|(_, ts)| ts))?);
    Ok(())
};

pub const MOVE_BACK: EuDef = |env| {
    let (t, ts) = env.arg("a0")?.pop_back()?;
    env.push(ts);
    env.push(EuType::opt(t));
    Ok(())
};

pub const MOVE_FRONT: EuDef = |env| {
    let (t, ts) = env.arg("a0")?.pop_front()?;
    env.push(ts);
    env.push(EuType::opt(t));
    Ok(())
};

pub const MOVE_INDEX: EuDef = |env| {
    let a1 = env.arg("a1 (index)").unwrap();
    let a0 = env.arg("a0").unwrap();
    let (t, ts) = a1.vecz1_2(|n| {
        a0.remove(n.try_isize()?)
            .map(|(t, ts)| (EuType::opt(t), ts))
    })?;
    env.push(ts);
    env.push(t);
    Ok(())
};

f_2_try_1!(DELETE);

pub const AT: EuDef = |env| {
    let a1 = env.arg("a1 (index)").unwrap();
    let a0 = env.arg("a0").unwrap();
    env.push(a1.vecz1(|n| a0.at(n.try_isize()?).map(EuType::opt))?);
    Ok(())
};

pub const TAKE: EuDef = |env| {
    let a1 = env.arg("a1 (int)")?;
    let a0 = env.arg("a0")?;
    env.push(a1.vecz1(|n| a0.take(n.try_isize()?))?);
    Ok(())
};

pub const DROP: EuDef = |env| {
    let a1 = env.arg("a1 (int)")?;
    let a0 = env.arg("a0")?;
    env.push(a1.vecz1(|n| a0.drop(n.try_isize()?))?);
    Ok(())
};

pub const CHUNK: EuDef = |env| {
    let a1 = env.arg("a1 (int)")?;
    let a0 = env.arg("a0")?;
    env.push(a1.vecz1(|n| a0.chunk(n.try_isize()?))?);
    Ok(())
};

pub const WINDOW: EuDef = |env| {
    let a1 = env.arg("a1 (int)")?;
    let a0 = env.arg("a0")?;
    env.push(a1.vecz1(|n| a0.window(n.try_usize()?))?);
    Ok(())
};

pub const DIVVY: EuDef = |env| {
    let a2 = env.arg("a2 (offset)")?;
    let a1 = env.arg("a1 (size)")?;
    let a0 = env.arg("a0")?;
    env.push(a1.vecz2(a2, |n, o| a0.divvy(n.try_usize()?, o.try_isize()?))?);
    Ok(())
};

pub const SORT: EuDef = |env| {
    let a0 = env.arg("a0")?;
    env.push(a0.sort()?);
    Ok(())
};

pub const FLAT: EuDef = |env| {
    let a0 = env.arg("a0")?;
    env.push(a0.flatten()?);
    Ok(())
};

pub const FLAT_REC: EuDef = |env| {
    let a0 = env.arg("a0")?;
    env.push(a0.flatten_rec()?);
    Ok(())
};

pub const ENUM: EuDef = |env| {
    let a0 = env.arg("a0")?;
    env.push(a0.enumerate());
    Ok(())
};

pub const PAIRS: EuDef = |env| {
    let a0 = env.arg("a0")?;
    env.push(a0.pairs());
    Ok(())
};

pub const MULTI_ZIP: EuDef = |env| {
    let a0 = env.arg("a0")?.to_vec()?;
    env.push(EuType::seq(EuType::multi_zip(a0)));
    Ok(())
};

pub const MULTI_CPROD: EuDef = |env| {
    let a0 = env.arg("a0")?.to_vec()?;
    env.push(EuType::seq(EuType::multi_cartesian_product(a0)));
    Ok(())
};

f_env_2_to_try_1!(MAP);
f_env_2_to_try_1!(MAP_ATOM);
f_env_2_to_try_1!(FLAT_MAP);
f_env_2_to_try_1!(FILTER);
f_env_2_to_try_1!(TAKE_WHILE);
f_env_2_to_try_1!(DROP_WHILE);
f_env_2_to_try_1!(FOLD1);
f_env_2_to_try_1!(SORT_BY);
f_env_2_to_try_1!(SORT_BY_KEY);
f_env_2_to_try_1!(FIND);
f_env_2_to_try_1!(ANY);
f_env_2_to_try_1!(ALL);

f_env_3_to_try_1!(ZIP, "");
f_env_3_to_try_1!(FOLD, " (acc)");
f_env_3_to_try_1!(SCAN, " (acc)");
f_env_3_to_try_1!(ZIP_ATOM, "");

f_2_try_1!(SEP);
