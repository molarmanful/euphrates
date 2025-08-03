use std::{
    iter,
    mem,
};

use ecow::eco_vec;
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
};

const NONE: EuDef = |env| {
    env.stack.push(EuType::Opt(None));
    Ok(())
};

const TRUE: EuDef = |env| {
    env.stack.push(EuType::Bool(true));
    Ok(())
};

const FALSE: EuDef = |env| {
    env.stack.push(EuType::Bool(false));
    Ok(())
};

const INF: EuDef = |env| {
    env.stack.push(EuType::F64(f64::INFINITY));
    Ok(())
};

const INF32: EuDef = |env| {
    env.stack.push(EuType::F32(f32::INFINITY));
    Ok(())
};

const SEQ_N0: EuDef = |env| {
    env.stack
        .push(EuType::Seq(EuType::iter_to_seq((0..).map(EuType::I128))));
    Ok(())
};

const DUP: EuDef = |env| {
    env.check_nargs(1)?;
    env.stack.push(env.stack.last().unwrap().clone());
    Ok(())
};

const DUPS: EuDef = |env| {
    env.stack.push(EuType::Vec(env.stack.clone()));
    Ok(())
};

const DUPD: EuDef = |env| {
    env.check_nargs(2)?;
    env.stack
        .insert(env.iflip(1), env.stack[env.iflip(1)].clone());
    Ok(())
};

const OVER: EuDef = |env| {
    env.check_nargs(2)?;
    env.stack.push(env.stack[env.iflip(1)].clone());
    Ok(())
};

const DDUP: EuDef = |env| {
    env.check_nargs(2)?;
    env.stack.push(env.stack[env.iflip(1)].clone());
    env.stack.push(env.stack[env.iflip(1)].clone());
    Ok(())
};

const EDUP: EuDef = |env| {
    env.check_nargs(3)?;
    for _ in 0..3 {
        env.stack.push(env.stack[env.iflip(2)].clone());
    }
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
    env.stack.remove(env.iflip(1));
    Ok(())
};

const PPOP: EuDef = |env| {
    env.check_nargs(2)?;
    env.stack.truncate(env.iflip(1));
    Ok(())
};

const QPOP: EuDef = |env| {
    env.check_nargs(3)?;
    env.stack.truncate(env.iflip(2));
    Ok(())
};

const SWAP: EuDef = |env| {
    env.check_nargs(2)?;
    let a = env.iflip(0);
    env.stack.make_mut().swap(a, a - 1);
    Ok(())
};

const REV: EuDef = |env| {
    env.stack.make_mut().reverse();
    Ok(())
};

const SWAPD: EuDef = |env| {
    env.check_nargs(3)?;
    let a = env.iflip(1);
    env.stack.make_mut().swap(a, a - 1);
    Ok(())
};

const TUCK: EuDef = |env| {
    env.check_nargs(2)?;
    env.stack
        .insert(env.iflip(1), env.stack.last().unwrap().clone());
    Ok(())
};

const ROT: EuDef = |env| {
    env.check_nargs(3)?;
    let a0 = env.stack.remove(env.iflip(2));
    env.stack.push(a0);
    Ok(())
};

const ROT_: EuDef = |env| {
    env.check_nargs(3)?;
    let a0 = env.stack.pop().unwrap();
    env.stack.insert(env.iflip(1), a0);
    Ok(())
};

const SOME: EuDef = |env| {
    let a0 = Box::new(env.pop()?);
    env.stack.push(EuType::Opt(Some(a0)));
    Ok(())
};

const OK: EuDef = |env| {
    let a0 = env.pop()?;
    env.stack.push(EuType::Res(Ok(Box::new(a0))));
    Ok(())
};

const ERR: EuDef = |env| {
    let a0 = env.pop()?;
    env.stack.push(EuType::Res(Err(Box::new(a0))));
    Ok(())
};

const TO_BOOL: EuDef = |env| {
    let a0 = env.pop()?;
    env.stack.push(EuType::Bool(a0.into()));
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
                env.stack
                    .push(EuType::Opt(a0.to_{{n}}().map(EuType::{{t}}).map(Box::new)));
                Ok(())
            };
        }
    }
}

gen_def_to_num!();

const TO_STR: EuDef = |env| {
    let a0 = match env.pop()? {
        t @ EuType::Str(_) => t,
        t => EuType::Str(t.to_string().into()),
    };
    env.stack.push(a0);
    Ok(())
};

const TO_VEC: EuDef = |env| {
    let a0 = env.pop()?.to_vec();
    env.stack.push(EuType::Vec(a0));
    Ok(())
};

const WRAP_VEC: EuDef = |env| {
    let a0 = env.pop()?;
    env.stack.push(EuType::Vec(eco_vec![a0]));
    Ok(())
};

const ALL_VEC: EuDef = |env| {
    let ts = EuType::Vec(mem::take(&mut env.stack));
    env.stack.push(ts);
    Ok(())
};

const EVAL_VEC: EuDef = |env| {
    let a0 = env.pop()?.to_expr()?;
    let mut env1 = EuEnv::from_iter(a0);
    env1.eval()?;
    env.stack.push(EuType::Vec(env1.stack));
    Ok(())
};

const TO_EXPR: EuDef = |env| {
    let a0 = env.pop()?.to_expr();
    env.stack.push(EuType::Res(
        a0.map(|ts| Box::new(EuType::Vec(ts)))
            .map_err(|e| Box::new(EuType::Str(e.to_string().into()))),
    ));
    Ok(())
};

const WRAP_EXPR: EuDef = |env| {
    let a0 = env.pop()?;
    env.stack.push(EuType::Expr(eco_vec![a0]));
    Ok(())
};

const TO_SEQ: EuDef = |env| {
    let a0 = env.pop()?.to_seq();
    env.stack.push(EuType::Seq(a0));
    Ok(())
};

const WRAP_SEQ: EuDef = |env| {
    let a0 = EuType::iter_to_seq(iter::once(env.pop()?));
    env.stack.push(EuType::Seq(a0));
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
    env.stack.push(a0);
    Ok(())
};

const NOT: EuDef = |env| {
    let a0: bool = env.pop()?.into();
    env.stack.push(EuType::Bool(!a0));
    Ok(())
};

const NEG: EuDef = |env| {
    let a0 = env.pop()?;
    env.stack.push(-a0);
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
                env.stack.push(a0 {{op}} a1);
                Ok(())
            };
        }
    }
}

gen_fn_math_binops!();
