use std::mem;

use ecow::eco_vec;
use phf::phf_map;

use super::EuDef;
use crate::types::EuType;

pub const CONSTS: phf::Map<&str, EuType> = phf_map! {
    "None" => EuType::Opt(None),
    "True" => EuType::Bool(true),
    "False" => EuType::Bool(false),
};

pub const CORE: phf::Map<&str, EuDef> = phf_map! {
    "dup" => DUP,
    "dups" => DUPS,
    "dupd" => DUPD,
    "over" => OVER,
    "ddup" => DDUP,
    "edup" => EDUP,
    "pop" => POP,
    "clr" => CLR,
    "nip" => NIP,
    "ppop" => PPOP,
    "qpop" => QPOP,
    "swap" => SWAP,
    "rev" => REV,
    "swapd" => SWAPD,
    "tuck" => TUCK,
    "rot" => ROT,
    "rot_" => ROT_,
    "Some" => SOME,
    "Ok" => OK,
    "Err" => ERR,
    "bool" => TO_BOOL,
    "isize" => TO_ISIZE,
    "i32" => TO_I32,
    "u32" => TO_U32,
    "f32" => TO_F32,
    "i64" => TO_I64,
    "u64" => TO_U64,
    "f64" => TO_F64,
    "i128" => TO_I128,
    "u128" => TO_U128,
    ">vec" => TO_VEC,
    "Vec" => WRAP_VEC,
    ">expr" => TO_EXPR,
    "Expr" => WRAP_EXPR,
    "#" => EVAL,
    "&#" => AND_EVAL,
    "|#" => OR_EVAL,
    "?#" => IF_EVAL,
    "pull" => PULL,
    "*pull" => ALL_PULL,
    "push" => PUSH,
    "*push" => ALL_PUSH,
    "->" => PULL_ARGS,
    "." => PUSH_1,
    "!" => NOT,
};

const DUP: EuDef = |env| {
    env.x.check_nargs(1)?;
    env.x.stack.push(env.x.stack.last().unwrap().clone());
    Ok(())
};

const DUPS: EuDef = |env| {
    env.x.stack.push(EuType::Vec(env.x.stack.clone()));
    Ok(())
};

const DUPD: EuDef = |env| {
    env.x.check_nargs(2)?;
    env.x
        .stack
        .insert(env.x.iflip(1), env.x.stack[env.x.iflip(1)].clone());
    Ok(())
};

const OVER: EuDef = |env| {
    env.x.check_nargs(2)?;
    env.x.stack.push(env.x.stack[env.x.iflip(1)].clone());
    Ok(())
};

const DDUP: EuDef = |env| {
    env.x.check_nargs(2)?;
    env.x.stack.push(env.x.stack[env.x.iflip(1)].clone());
    env.x.stack.push(env.x.stack[env.x.iflip(1)].clone());
    Ok(())
};

const EDUP: EuDef = |env| {
    env.x.check_nargs(3)?;
    for _ in 0..3 {
        env.x.stack.push(env.x.stack[env.x.iflip(2)].clone());
    }
    Ok(())
};

const POP: EuDef = |env| {
    env.x.pop()?;
    Ok(())
};

const CLR: EuDef = |env| {
    env.x.stack.clear();
    Ok(())
};

const NIP: EuDef = |env| {
    env.x.check_nargs(2)?;
    env.x.stack.remove(env.x.iflip(1));
    Ok(())
};

const PPOP: EuDef = |env| {
    env.x.check_nargs(2)?;
    env.x.stack.truncate(env.x.iflip(1));
    Ok(())
};

const QPOP: EuDef = |env| {
    env.x.check_nargs(3)?;
    env.x.stack.truncate(env.x.iflip(2));
    Ok(())
};

const SWAP: EuDef = |env| {
    env.x.check_nargs(2)?;
    let a = env.x.iflip(0);
    env.x.stack.make_mut().swap(a, a - 1);
    Ok(())
};

const REV: EuDef = |env| {
    env.x.stack.make_mut().reverse();
    Ok(())
};

const SWAPD: EuDef = |env| {
    env.x.check_nargs(3)?;
    let a = env.x.iflip(1);
    env.x.stack.make_mut().swap(a, a - 1);
    Ok(())
};

const TUCK: EuDef = |env| {
    env.x.check_nargs(2)?;
    env.x
        .stack
        .insert(env.x.iflip(1), env.x.stack.last().unwrap().clone());
    Ok(())
};

const ROT: EuDef = |env| {
    env.x.check_nargs(3)?;
    let a0 = env.x.stack.remove(env.x.iflip(2));
    env.x.stack.push(a0);
    Ok(())
};

const ROT_: EuDef = |env| {
    env.x.check_nargs(3)?;
    let a0 = env.x.stack.pop().unwrap();
    env.x.stack.insert(env.x.iflip(1), a0);
    Ok(())
};

const SOME: EuDef = |env| {
    let a0 = Box::new(env.x.pop()?);
    env.x.stack.push(EuType::Opt(Some(a0)));
    Ok(())
};

const OK: EuDef = |env| {
    let a0 = env.x.pop()?;
    env.x.stack.push(EuType::Res(Ok(Box::new(a0))));
    Ok(())
};

const ERR: EuDef = |env| {
    let a0 = env.x.pop()?;
    env.x.stack.push(EuType::Res(Err(Box::new(a0))));
    Ok(())
};

const TO_BOOL: EuDef = |env| {
    let a0 = env.x.pop()?;
    env.x.stack.push(EuType::Bool(a0.into()));
    Ok(())
};

#[crabtime::function]
fn gen_def_to_num() {
    let types = [
        "Isize", "Usize", "I32", "U32", "F32", "I64", "U64", "F64", "I128", "U128",
    ];
    for &t in &types {
        let n = t.to_lowercase();
        let n_up = t.to_uppercase();
        crabtime::output! {
            const TO_{{n_up}}: EuDef = |env| {
                let x = env.x.pop()?;
                env.x.stack
                    .push(EuType::Opt(x.to_{{n}}().map(EuType::{{t}}).map(Box::new)));
                Ok(())
            };
        }
    }
}

gen_def_to_num!();

const WRAP_VEC: EuDef = |env| {
    let a0 = env.x.pop()?;
    env.x.stack.push(EuType::Vec(eco_vec![a0]));
    Ok(())
};

const TO_VEC: EuDef = |env| {
    let a0 = env.x.pop()?.to_vec();
    env.x.stack.push(EuType::Vec(a0));
    Ok(())
};

const WRAP_EXPR: EuDef = |env| {
    let a0 = env.x.pop()?;
    env.x.stack.push(EuType::Expr(eco_vec![a0]));
    Ok(())
};

const TO_EXPR: EuDef = |env| {
    let a0 = env.x.pop()?.to_expr();
    env.x.stack.push(EuType::Res(
        a0.map(|ts| Box::new(EuType::Vec(ts)))
            .map_err(|e| Box::new(EuType::Str(e.to_string().into()))),
    ));
    Ok(())
};

const EVAL: EuDef = |env| {
    let a0 = env.x.pop()?.to_expr()?;
    env.eval_iter_scoped(a0)
};

const AND_EVAL: EuDef = |env| {
    env.x.check_nargs(2)?;
    let a1 = env.x.pop().unwrap().to_expr()?;
    let a0 = env.x.pop().unwrap().into();
    if a0 { env.eval_iter_scoped(a1) } else { Ok(()) }
};

const OR_EVAL: EuDef = |env| {
    env.x.check_nargs(2)?;
    let a1 = env.x.pop().unwrap().to_expr()?;
    let a0 = env.x.pop().unwrap().into();
    if a0 { Ok(()) } else { env.eval_iter_scoped(a1) }
};

const IF_EVAL: EuDef = |env| {
    env.x.check_nargs(3)?;
    let a2 = env.x.pop().unwrap().to_expr()?;
    let a1 = env.x.pop().unwrap().to_expr()?;
    let a0: bool = env.x.pop().unwrap().into();
    env.eval_iter_scoped(if a0 { a1 } else { a2 })
};

const PULL: EuDef = |env| {
    let a0 = env.x.pop()?.to_res_usize()?;
    let prev = env.parent()?;
    let tx = prev.split_off(a0)?;
    env.x.stack.extend(tx);
    Ok(())
};

const ALL_PULL: EuDef = |env| {
    let tx = mem::take(&mut env.parent()?.stack);
    env.x.stack.extend(tx);
    Ok(())
};

const PUSH: EuDef = |env| {
    let a0 = env.x.pop()?.to_res_usize()?;
    let tx = env.x.split_off(a0)?;
    env.parent()?.stack.extend(tx);
    Ok(())
};

const ALL_PUSH: EuDef = |env| {
    let tx = mem::take(&mut env.x.stack);
    env.parent()?.stack.extend(tx);
    Ok(())
};

const PULL_ARGS: EuDef = |env| {
    let a0 = env.x.pop()?.to_expr()?;
    env.pull_args(a0)
};

const PUSH_1: EuDef = |env| {
    let a0 = env.x.pop()?;
    env.parent()?.stack.push(a0);
    Ok(())
};

const NOT: EuDef = |env| {
    let a0: bool = env.x.pop()?.into();
    env.x.stack.push(EuType::Bool(!a0));
    Ok(())
};
