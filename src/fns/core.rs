use std::{
    iter,
    mem,
};

use ordered_float::{
    FloatCore,
    OrderedFloat,
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
    "MinI32" => MIN_I32,
    "MaxI32" => MAX_I32,
    "MinI64" => MIN_I64,
    "MaxI64" => MAX_I64,
    "MinF32" => MIN_F32,
    "MaxF32" => MAX_F32,
    "MinF64" => MIN_F64,
    "MaxF64" => MAX_F64,
    "Inf" => INF,
    "Inf32" => INF32,
    "NaN" => NAN,
    "NaN32" => NAN32,
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

    "print" => PRINT,
    "println" => PRINTLN,

    "bool" => TO_BOOL,
    "i32" => TO_I32,
    "f32" => TO_F32,
    "i64" => TO_I64,
    "f64" => TO_F64,

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

    "cmp" => CMP,
    "=" => EQ,
    "<" => LT,
    "<=" => LE,
    ">" => GT,
    ">=" => GE,

    "_" => NEG,
    "+" => ADD,
    "-" => SUB,
    "*" => MUL,
    "/" => DIV,

    "tk" => TAKE,
    "dp" => DROP,
    "sort" => SORT,

    "map" => MAP,
    "mapf" => MAPF,
    "flat" => FLAT,
    "flat*" => FLAT_REC,
    "zip" => ZIP,
    "fold" => FOLD,
    "scan" => SCAN,
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

#[crabtime::function]
fn gen_fn_int_consts() {
    let types = ["I32", "I64"];
    let consts = ["MIN", "MAX"];
    for t in types {
        let n = t.to_lowercase();
        for c in consts {
            crabtime::output! {
                const {{c}}_{{t}}: EuDef = |env| {
                    env.push(EuType::{{t}}({{n}}::{{c}}));
                    Ok(())
                };
            };
        }
    }
}

gen_fn_int_consts!();

#[crabtime::function]
fn gen_fn_float_consts() {
    let types = ["F32", "F64"];
    let consts = [("MIN", "min_value"), ("MAX", "max_value")];
    for t in types {
        for (c, f) in consts {
            crabtime::output! {
                const {{c}}_{{t}}: EuDef = |env| {
                    env.push(EuType::{{t}}(OrderedFloat::{{f}}()));
                    Ok(())
                };
            };
        }
    }
}

gen_fn_float_consts!();

const INF: EuDef = |env| {
    env.push(EuType::F64(OrderedFloat::infinity()));
    Ok(())
};

const INF32: EuDef = |env| {
    env.push(EuType::F32(OrderedFloat::infinity()));
    Ok(())
};

const NAN: EuDef = |env| {
    env.push(EuType::F64(OrderedFloat::nan()));
    Ok(())
};

const NAN32: EuDef = |env| {
    env.push(EuType::F32(OrderedFloat::nan()));
    Ok(())
};

const SEQ_N0: EuDef = |env| {
    env.push(EuType::seq((0..).map(EuType::i64).map(Ok)));
    Ok(())
};

const DUP: EuDef = |env| {
    env.check_nargs(1)?;
    env.push(env.last().unwrap().clone());
    Ok(())
};

const DUPS: EuDef = |env| {
    env.push(EuType::vec(env.stack.clone()));
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
    env.stack.make_mut().swap(a, a - 1);
    Ok(())
};

const REV: EuDef = |env| {
    env.stack.make_mut().reverse();
    Ok(())
};

const SWAPD: EuDef = |env| {
    env.check_nargs(3)?;
    let a = env.iflip(1).unwrap();
    env.stack.make_mut().swap(a, a - 1);
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
    env.stack.make_mut().swap(i, j);
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
    let a0 = env.stack.pop().unwrap();
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
    let a0 = env.stack.pop().unwrap().to_res_isize()?;
    let i = env.iflip(a0)?;
    let t = env.stack.pop().unwrap();
    env.stack.insert(i, t);
    Ok(())
};

const PRINT: EuDef = |env| {
    print!("{}", env.pop()?);
    Ok(())
};

const PRINTLN: EuDef = |env| {
    println!("{}", env.pop()?);
    Ok(())
};

const SOME: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::opt(Some(a0)));
    Ok(())
};

const OK: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::res(Ok(a0)));
    Ok(())
};

const ERR: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::res(Err(a0)));
    Ok(())
};

const TO_BOOL: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::Bool(a0.into()));
    Ok(())
};

#[crabtime::function]
fn gen_def_to_num() {
    let types = ["I32", "I64", "F32", "F64"];
    for &t in &types {
        let n = t.to_lowercase();
        let n_up = t.to_uppercase();
        crabtime::output! {
            const TO_{{n_up}}: EuDef = |env| {
                let a0 = env.pop()?;
                env.push(EuType::opt(a0.to_{{n}}().map(EuType::{{n}})));
                Ok(())
            };
        }
    }
}

gen_def_to_num!();

const TO_STR: EuDef = |env| {
    let a0 = match env.pop()? {
        t @ EuType::Str(_) => t,
        t => EuType::str(t.to_string()),
    };
    env.push(a0);
    Ok(())
};

const TO_VEC: EuDef = |env| {
    let a0 = env.pop()?.to_vec()?;
    env.push(EuType::vec(a0));
    Ok(())
};

const WRAP_VEC: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::vec([a0]));
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
    env.push(EuType::res_str(a0.map(EuType::vec)));
    Ok(())
};

const WRAP_EXPR: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::expr([a0]));
    Ok(())
};

const TO_SEQ: EuDef = |env| {
    let a0 = env.pop()?.to_seq();
    env.push(EuType::Seq(a0));
    Ok(())
};

const WRAP_SEQ: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::seq(iter::once(Ok(a0))));
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

const CMP: EuDef = |env| {
    let a1 = env.pop()?;
    let a0 = env.pop()?;
    env.push(EuType::i32(a0.cmp(&a1) as i32));
    Ok(())
};

#[crabtime::function]
fn gen_fn_cmp_binops() {
    for (name, op) in [
        ("EQ", "=="),
        ("NE", "!="),
        ("LT", "<"),
        ("LE", "<="),
        ("GT", ">"),
        ("GE", ">="),
    ] {
        crabtime::output! {
            const {{name}}: EuDef = |env| {
                env.check_nargs(2)?;
                let a1 = env.pop().unwrap();
                let a0 = env.pop().unwrap();
                env.push(EuType::Bool(a0 {{op}} a1));
                Ok(())
            };
        }
    }
}

gen_fn_cmp_binops!();

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
                env.push((a0 {{op}} a1)?);
                Ok(())
            };
        }
    }
}

gen_fn_math_binops!();

const TAKE: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.pop()?.to_seq();
    env.push(a1.map(move |n| {
        let n = n.to_res_usize()?;
        Ok(EuType::seq(a0.clone().take(n)))
    })?);
    Ok(())
};

const DROP: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.pop()?.to_seq();
    env.push(a1.map(move |n| {
        let n = n.to_res_usize()?;
        Ok(EuType::seq(a0.clone().skip(n)))
    })?);
    Ok(())
};

const SORT: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.sorted()?);
    Ok(())
};

const MAP: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    let scope = env.scope.clone();
    env.push(if a1.is_many() {
        a1.map(move |f| a0.clone().map_env(f, scope.clone()))
    } else {
        a1.map_once(|f| a0.map_env(f, scope))
    }?);
    Ok(())
};

const MAPF: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    let scope = env.scope.clone();
    env.push(if a1.is_many() {
        a1.map(move |f| a0.clone().flat_map_env(f, scope.clone()))
    } else {
        a1.map_once(|f| a0.flat_map_env(f, scope))
    }?);
    Ok(())
};

const FLAT: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.flatten()?);
    Ok(())
};

const FLAT_REC: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.flatten_rec()?);
    Ok(())
};

const ZIP: EuDef = |env| {
    env.check_nargs(3)?;
    let a2 = env.stack.pop().unwrap();
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    let scope = env.scope.clone();
    env.push(if a2.is_many() {
        a2.map(move |f| a0.clone().zip_env(a1.clone(), f, scope.clone()))
    } else {
        a2.map_once(|f| a0.zip_env(a1, f, scope))
    }?);
    Ok(())
};

const FOLD: EuDef = |env| {
    env.check_nargs(3)?;
    let a2 = env.stack.pop().unwrap();
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    let scope = env.scope.clone();
    env.push(if a2.is_many() {
        a2.map(move |f| a0.clone().fold_env(a1.clone(), f, scope.clone()))
    } else {
        a2.map_once(|f| a0.fold_env(a1, f, scope))
    }?);
    Ok(())
};

const SCAN: EuDef = |env| {
    env.check_nargs(3)?;
    let a2 = env.stack.pop().unwrap();
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    let scope = env.scope.clone();
    env.push(if a2.is_many() {
        a2.map(move |f| a0.clone().scan_env(a1.clone(), f, scope.clone()))
    } else {
        a2.map_once(|f| a0.scan_env(a1, f, scope))
    }?);
    Ok(())
};
