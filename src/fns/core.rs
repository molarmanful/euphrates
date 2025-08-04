use std::{
    iter,
    mem,
};

use phf::phf_map;

use super::EuDef;
use crate::{
    env::EuEnv,
    types::EuType,
};

pub const CORE: phf::Map<&str, EuDef> = phf_map! {
    "None" => NONE,
    "True" => TRUE,
    "False" => FALSE,
    "inf" => INF,
    "inf32" => INF32,
    "SeqN0" => SEQ_N0,

    "dup" => DUP,
    "dups" => DUPS,
    "dupd" => DUPD,
    "over" => OVER,
    "ddup" => DDUP,
    "edup" => EDUP,
    "pick" => PICK,
    "pop" => POP,
    "clr" => CLR,
    "nip" => NIP,
    "ppop" => PPOP,
    "qpop" => QPOP,
    "nix" => NIX,
    "swap" => SWAP,
    "rev" => REV,
    "swapd" => SWAPD,
    "tuck" => TUCK,
    "trade" => TRADE,
    "rot" => ROT,
    "rot_" => ROT_,
    "roll" => ROLL,
    "roll_" => ROLL_,

    "bool" => TO_BOOL,
    "i32" => TO_I32,
    "f32" => TO_F32,
    "i64" => TO_I64,
    "f64" => TO_F64,
    "i128" => TO_I128,

    ">str" => TO_STR,

    "Some" => SOME,

    "Ok" => OK,
    "Err" => ERR,

    ">vec" => TO_VEC,
    "Vec" => WRAP_VEC,
    "*vec" => ALL_VEC,
    "#vec" => EVAL_VEC,

    ">expr" => TO_EXPR,
    "Expr" => WRAP_EXPR,

    ">seq" => TO_SEQ,
    "Seq" => WRAP_SEQ,

    "#" => EVAL,
    "&#" => AND_EVAL,
    "|#" => OR_EVAL,
    "?#" => IF_EVAL,

    "->" => BIND_ARGS,
    "?" => TRY,

    "!" => NOT,

    "_" => NEG,
    "+" => ADD,
    "-" => SUB,
    "*" => MUL,
    "/" => DIV,

    "tk" => TAKE,
    "dp" => DROP,

    "map" => MAP,
};

const NONE: EuDef = |env| {
    env.push(EuType::Opt(None));
    Ok(())
};

const TRUE: EuDef = |env| {
    env.push(EuType::Bool(true));
    Ok(())
};

const FALSE: EuDef = |env| {
    env.push(EuType::Bool(false));
    Ok(())
};

const INF: EuDef = |env| {
    env.push(EuType::F64(f64::INFINITY));
    Ok(())
};

const INF32: EuDef = |env| {
    env.push(EuType::F32(f32::INFINITY));
    Ok(())
};

const SEQ_N0: EuDef = |env| {
    env.push(EuType::Seq(EuType::seq((0..).map(EuType::I128))));
    Ok(())
};

const DUP: EuDef = |env| {
    env.check_nargs(1)?;
    env.push(env.last().unwrap().clone());
    Ok(())
};

const DUPS: EuDef = |env| {
    env.push(EuType::Vec(Box::new(env.stack.clone())));
    Ok(())
};

const DUPD: EuDef = |env| {
    env.check_nargs(2)?;
    env.stack.insert(
        env.iflip(1).unwrap(),
        env.stack[env.iflip(1).unwrap()].clone(),
    );
    Ok(())
};

const OVER: EuDef = |env| {
    env.check_nargs(2)?;
    env.push(env.stack[env.iflip(1).unwrap()].clone());
    Ok(())
};

const DDUP: EuDef = |env| {
    env.check_nargs(2)?;
    env.push(env.stack[env.iflip(1).unwrap()].clone());
    env.push(env.stack[env.iflip(1).unwrap()].clone());
    Ok(())
};

const EDUP: EuDef = |env| {
    env.check_nargs(3)?;
    for _ in 0..3 {
        env.push(env.stack[env.iflip(2).unwrap()].clone());
    }
    Ok(())
};

const PICK: EuDef = |env| {
    let a0 = env.pop()?.to_res_isize()?;
    env.push(env.stack[env.iflip(a0)?].clone());
    Ok(())
};

const POP: EuDef = |env| {
    env.pop()?;
    Ok(())
};

const CLR: EuDef = |env| {
    env.stack.clear();
    Ok(())
};

const NIP: EuDef = |env| {
    env.check_nargs(2)?;
    env.stack.remove(env.iflip(1).unwrap());
    Ok(())
};

const PPOP: EuDef = |env| {
    env.check_nargs(2)?;
    env.stack.truncate(env.iflip(1).unwrap());
    Ok(())
};

const QPOP: EuDef = |env| {
    env.check_nargs(3)?;
    env.stack.truncate(env.iflip(2).unwrap());
    Ok(())
};

const NIX: EuDef = |env| {
    let a0 = env.pop()?.to_res_isize()?;
    env.stack.remove(env.iflip(a0)?);
    Ok(())
};

const SWAP: EuDef = |env| {
    env.check_nargs(2)?;
    let a = env.iflip(0).unwrap();
    env.stack.swap(a, a - 1);
    Ok(())
};

const REV: EuDef = |env| {
    env.stack = env.stack.clone().into_iter().rev().collect();
    Ok(())
};

const SWAPD: EuDef = |env| {
    env.check_nargs(3)?;
    let a = env.iflip(1).unwrap();
    env.stack.swap(a, a - 1);
    Ok(())
};

const TUCK: EuDef = |env| {
    env.check_nargs(2)?;
    env.stack
        .insert(env.iflip(1).unwrap(), env.stack.last().unwrap().clone());
    Ok(())
};

const TRADE: EuDef = |env| {
    let a0 = env.pop()?.to_res_isize()?;
    let i = env.iflip(a0)?;
    let j = env.iflip(0).unwrap();
    env.stack.swap(i, j);
    Ok(())
};

const ROT: EuDef = |env| {
    env.check_nargs(3)?;
    let a0 = env.stack.remove(env.iflip(2).unwrap());
    env.push(a0);
    Ok(())
};

const ROT_: EuDef = |env| {
    env.check_nargs(3)?;
    let a0 = env.stack.pop_back().unwrap();
    env.stack.insert(env.iflip(1).unwrap(), a0);
    Ok(())
};

const ROLL: EuDef = |env| {
    let a0 = env.pop()?.to_res_isize()?;
    let t = env.stack.remove(env.iflip(a0)?);
    env.push(t);
    Ok(())
};

const ROLL_: EuDef = |env| {
    env.check_nargs(2)?;
    let a0 = env.stack.pop_back().unwrap().to_res_isize()?;
    let i = env.iflip(a0)?;
    let t = env.stack.pop_back().unwrap();
    env.stack.insert(i, t);
    Ok(())
};

const SOME: EuDef = |env| {
    let a0 = Box::new(env.pop()?);
    env.push(EuType::Opt(Some(a0)));
    Ok(())
};

const OK: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::Res(Ok(Box::new(a0))));
    Ok(())
};

const ERR: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::Res(Err(Box::new(a0))));
    Ok(())
};

const TO_BOOL: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::Bool(a0.into()));
    Ok(())
};

#[crabtime::function]
fn gen_def_to_num() {
    let types = ["I32", "F32", "I64", "F64", "I128"];
    for &t in &types {
        let n = t.to_lowercase();
        let n_up = t.to_uppercase();
        crabtime::output! {
            const TO_{{n_up}}: EuDef = |env| {
                let a0 = env.pop()?;
                env.push(EuType::Opt(a0.to_{{n}}().map(EuType::{{t}}).map(Box::new)));
                Ok(())
            };
        }
    }
}

gen_def_to_num!();

const TO_STR: EuDef = |env| {
    let a0 = match env.pop()? {
        t @ EuType::Str(_) => t,
        t => EuType::Str(Box::new(t.to_string().into())),
    };
    env.push(a0);
    Ok(())
};

const TO_VEC: EuDef = |env| {
    let a0 = env.pop()?.to_vec();
    env.push(EuType::Vec(Box::new(a0)));
    Ok(())
};

const WRAP_VEC: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::vec(imbl::vector![a0]));
    Ok(())
};

const ALL_VEC: EuDef = |env| {
    let ts = EuType::vec(mem::take(&mut env.stack));
    env.push(ts);
    Ok(())
};

const EVAL_VEC: EuDef = |env| {
    let a0 = env.pop()?.to_expr()?;
    env.push(EuType::vec(EuEnv::apply(a0, &[], env.scope.clone())?.stack));
    Ok(())
};

const TO_EXPR: EuDef = |env| {
    let a0 = env.pop()?.to_expr();
    env.push(EuType::Res(
        a0.map(|ts| Box::new(EuType::vec(ts)))
            .map_err(|e| Box::new(EuType::str(e.to_string()))),
    ));
    Ok(())
};

const WRAP_EXPR: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::expr(imbl::vector![a0]));
    Ok(())
};

const TO_SEQ: EuDef = |env| {
    let a0 = env.pop()?.to_seq();
    env.push(EuType::Seq(a0));
    Ok(())
};

const WRAP_SEQ: EuDef = |env| {
    let a0 = EuType::seq(iter::once(env.pop()?));
    env.push(EuType::Seq(a0));
    Ok(())
};

const EVAL: EuDef = |env| {
    let a0 = env.pop()?.to_expr()?;
    env.eval_iter(a0)
};

const AND_EVAL: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.pop().unwrap().to_expr()?;
    let a0 = env.pop().unwrap().into();
    if a0 { env.eval_iter(a1) } else { Ok(()) }
};

const OR_EVAL: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.pop().unwrap().to_expr()?;
    let a0 = env.pop().unwrap().into();
    if a0 { Ok(()) } else { env.eval_iter(a1) }
};

const IF_EVAL: EuDef = |env| {
    env.check_nargs(3)?;
    let a2 = env.pop().unwrap().to_expr()?;
    let a1 = env.pop().unwrap().to_expr()?;
    let a0: bool = env.pop().unwrap().into();
    env.eval_iter(if a0 { a1 } else { a2 })
};

const BIND_ARGS: EuDef = |env| {
    let a0 = env.pop()?.to_expr()?;
    env.bind_args(a0)
};

const TRY: EuDef = |env| {
    let a0 = match env.pop()? {
        t @ (EuType::Opt(None) | EuType::Res(Err(_))) => {
            env.clear_queue();
            t
        }
        EuType::Opt(Some(t)) | EuType::Res(Ok(t)) => *t,
        t => t,
    };
    env.push(a0);
    Ok(())
};

const NOT: EuDef = |env| {
    let a0: bool = env.pop()?.into();
    env.push(EuType::Bool(!a0));
    Ok(())
};

const NEG: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(-a0);
    Ok(())
};

#[crabtime::function]
fn gen_fn_math_binops() {
    for (name, op) in [("ADD", "+"), ("SUB", "-"), ("MUL", "*"), ("DIV", "/")] {
        crabtime::output! {
            const {{name}}: EuDef = |env| {
                env.check_nargs(2)?;
                let a1 = env.pop().unwrap();
                let a0 = env.pop().unwrap();
                env.push(a0 {{op}} a1);
                Ok(())
            };
        }
    }
}

gen_fn_math_binops!();

const TAKE: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop_back().unwrap().to_res_usize()?;
    let a0 = env.pop()?.to_seq();
    {
        let mut guard = a0.lock().unwrap();
        *guard = Box::new(EuType::take_iter(&mut guard).take(a1));
    }
    env.push(EuType::Seq(a0));
    Ok(())
};

const DROP: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop_back().unwrap().to_res_usize()?;
    let a0 = env.pop()?.to_seq();
    {
        let mut guard = a0.lock().unwrap();
        *guard = Box::new(EuType::take_iter(&mut guard).skip(a1));
    }
    env.push(EuType::Seq(a0));
    Ok(())
};

const MAP: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop_back().unwrap().to_expr()?;
    let a0 = env.pop()?;
    let scope = env.scope.clone();
    let res = a0.map(move |t| {
        EuType::Res(
            EuEnv::apply(a1.clone(), &[t], scope.clone())
                .and_then(|mut env| env.pop().map(Box::new))
                .map_err(|e| Box::new(EuType::Str(Box::new(e.to_string().into())))),
        )
    });
    env.push(res);
    Ok(())
};
