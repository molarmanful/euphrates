use crate::{
    fns::{
        EuDef,
        macros::{
            f_2_to_1,
            f_env_2_to_1,
            f_env_3_to_1,
        },
    },
    types::EuType,
};

pub const GET: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(EuType::opt(a0.get(&a1)?));
    Ok(())
};

pub const HAS: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(EuType::Bool(a0.has(&a1)));
    Ok(())
};

f_2_to_1!(PUSH_BACK);
f_2_to_1!(PUSH_FRONT);

pub const INSERT: EuDef = |env| {
    env.check_nargs(3)?;
    let a2 = env.stack.pop().unwrap();
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a2.vecz1(|n| a0.insert(n.try_isize()?, a1))?);
    Ok(())
};

f_2_to_1!(APPEND);

pub const POP_BACK: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.pop_back()?.1);
    Ok(())
};

pub const POP_FRONT: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.pop_front()?.1);
    Ok(())
};

pub const REMOVE_INDEX: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a1.vecz1(|n| a0.remove(n.try_isize()?).map(|(_, ts)| ts))?);
    Ok(())
};

pub const MOVE_BACK: EuDef = |env| {
    let (t, ts) = env.pop()?.pop_back()?;
    env.push(ts);
    env.push(EuType::opt(t));
    Ok(())
};

pub const MOVE_FRONT: EuDef = |env| {
    let (t, ts) = env.pop()?.pop_front()?;
    env.push(ts);
    env.push(EuType::opt(t));
    Ok(())
};

pub const MOVE_INDEX: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    let (t, ts) = a1.vecz1_2(|n| {
        a0.remove(n.try_isize()?)
            .map(|(t, ts)| (EuType::opt(t), ts))
    })?;
    env.push(ts);
    env.push(t);
    Ok(())
};

f_2_to_1!(DELETE);

pub const AT: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a1.vecz1(|n| a0.at(n.try_isize()?).map(EuType::opt))?);
    Ok(())
};

pub const TAKE: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a1.vecz1(|n| a0.take(n.try_isize()?))?);
    Ok(())
};

pub const DROP: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a1.vecz1(|n| a0.drop(n.try_isize()?))?);
    Ok(())
};

pub const CHUNK: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a1.vecz1(|n| a0.chunk(n.try_isize()?))?);
    Ok(())
};

pub const WINDOW: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a1.vecz1(|n| a0.window(n.try_usize()?))?);
    Ok(())
};

pub const DIVVY: EuDef = |env| {
    env.check_nargs(3)?;
    let a2 = env.stack.pop().unwrap();
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a1.vecz2(a2, |n, o| a0.divvy(n.try_usize()?, o.try_isize()?))?);
    Ok(())
};

pub const SORT: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.sort()?);
    Ok(())
};

pub const FLAT: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.flatten()?);
    Ok(())
};

pub const FLAT_REC: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.flatten_rec()?);
    Ok(())
};

pub const ENUM: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.enumerate());
    Ok(())
};

pub const PAIRS: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.pairs());
    Ok(())
};

pub const MULTI_ZIP: EuDef = |env| {
    let a0 = env.pop()?.to_vec()?;
    env.push(EuType::seq(EuType::multi_zip(a0)));
    Ok(())
};

pub const MULTI_CPROD: EuDef = |env| {
    let a0 = env.pop()?.to_vec()?;
    env.push(EuType::seq(EuType::multi_cartesian_product(a0)));
    Ok(())
};

f_env_2_to_1!(MAP);
f_env_2_to_1!(MAP_ATOM);
f_env_2_to_1!(FLAT_MAP);
f_env_2_to_1!(FILTER);
f_env_2_to_1!(TAKE_WHILE);
f_env_2_to_1!(DROP_WHILE);
f_env_2_to_1!(FOLD1);
f_env_2_to_1!(SORT_BY);
f_env_2_to_1!(SORT_BY_KEY);
f_env_2_to_1!(FIND);
f_env_2_to_1!(ANY);
f_env_2_to_1!(ALL);

f_env_3_to_1!(ZIP);
f_env_3_to_1!(FOLD);
f_env_3_to_1!(SCAN);
f_env_3_to_1!(ZIP_ATOM);

f_2_to_1!(SEP);
