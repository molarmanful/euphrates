use super::EuDef;
use crate::types::EuType;

pub const GET: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(EuType::opt(a0.get(a1)?));
    Ok(())
};

pub const HAS: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(EuType::Bool(a0.has(&a1)));
    Ok(())
};

pub const PUSH_BACK: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.push_back(a1)?);
    Ok(())
};

pub const PUSH_FRONT: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.push_front(a1)?);
    Ok(())
};

pub const INSERT: EuDef = |env| {
    env.check_nargs(3)?;
    let a2 = env.stack.pop().unwrap();
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a2.vecz1(|n| a0.insert(n.try_isize()?, a1))?);
    Ok(())
};

pub const APPEND: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.append(a1)?);
    Ok(())
};

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

pub const DELETE: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.delete(a1));
    Ok(())
};

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
    env.push(a0.sorted()?);
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

pub const MAP: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.map_env(a1, env.scope.clone())?);
    Ok(())
};

pub const MAP_ATOM: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.map_atom_env(a1, env.scope.clone())?);
    Ok(())
};

pub const FLATMAP: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.flat_map_env(a1, env.scope.clone())?);
    Ok(())
};

pub const FILTER: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.filter_env(a1, env.scope.clone())?);
    Ok(())
};

pub const TAKE_WHILE: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.take_while_env(a1, env.scope.clone())?);
    Ok(())
};

pub const DROP_WHILE: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.drop_while_env(a1, env.scope.clone())?);
    Ok(())
};

pub const ZIP: EuDef = |env| {
    env.check_nargs(3)?;
    let a2 = env.stack.pop().unwrap();
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.zip_env(a1, a2, env.scope.clone())?);
    Ok(())
};

pub const ZIP_ATOM: EuDef = |env| {
    env.check_nargs(2)?;
    let a2 = env.stack.pop().unwrap();
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.zip_atom_env(a1, a2, env.scope.clone())?);
    Ok(())
};

pub const FOLD: EuDef = |env| {
    env.check_nargs(3)?;
    let a2 = env.stack.pop().unwrap();
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.fold_env(a1, a2, env.scope.clone())?);
    Ok(())
};

pub const FOLD1: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.fold1_env(a1, env.scope.clone())?);
    Ok(())
};

pub const SCAN: EuDef = |env| {
    env.check_nargs(3)?;
    let a2 = env.stack.pop().unwrap();
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.scan_env(a1, a2, env.scope.clone())?);
    Ok(())
};

pub const SORT_BY: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.sorted_by_env(a1, env.scope.clone())?);
    Ok(())
};

pub const SORT_BY_KEY: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.sorted_by_key_env(a1, env.scope.clone())?);
    Ok(())
};

pub const FIND: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.find_env(a1, env.scope.clone())?);
    Ok(())
};

pub const ANY: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.any_env(a1, env.scope.clone())?);
    Ok(())
};

pub const ALL: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.all_env(a1, env.scope.clone())?);
    Ok(())
};

pub const SEP: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.sep(a1)?);
    Ok(())
};
