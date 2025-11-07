use std::{
    io,
    iter,
    mem,
    ops::{
        Add,
        Div,
        Mul,
        Rem,
        Sub,
    },
};

use anyhow::anyhow;
use num_traits::Pow;
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
    "Stack" => STACK,

    "dup" => DUP,
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
    "," => PAIR,
    "wrap" => WRAP,
    "wrap_" => WRAP_,
    "usurp" => USURP,
    "sub" => SUB_STACK,
    "dip" => DIP,

    "read" => READ,
    "readln" => READLN,
    "print" => PRINT,
    "println" => PRINTLN,

    "bool" => TO_BOOL,
    "i32" => TO_I32,
    "f32" => TO_F32,
    "i64" => TO_I64,
    "f64" => TO_F64,
    "ibig" => TO_IBIG,

    ">str" => TO_STR,

    "Some" => SOME,

    "Ok" => OK,
    "Err" => ERR,
    "#res" => EVAL_RES,

    ">vec" => TO_VEC,
    "Vec" => WRAP_VEC,
    "*vec" => ALL_VEC,
    "#vec" => EVAL_VEC,

    ">map" => TO_MAP,
    "Map" => WRAP_MAP,
    "*map" => ALL_MAP,
    "#map" => EVAL_MAP,

    ">set" => TO_SET,
    "Set" => WRAP_SET,
    "*set" => ALL_SET,
    "#set" => EVAL_SET,

    ">expr" => TO_EXPR,
    "Expr" => WRAP_EXPR,

    ">seq" => TO_SEQ,
    "Seq" => WRAP_SEQ,
    "fold_" => FOLD_,
    "rpt" => RPT,
    "cyc" => CYC,

    "#" => EVAL,
    "tap" => TAP,
    "&#" => AND_EVAL,
    "|#" => OR_EVAL,
    "&|#" => IF_EVAL,

    "->" => BIND_ARGS,
    "?" => TRY,

    "!" => NOT,

    "<=>" => CMP,
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
    "%" => REM,
    "^" => POW,

    ":" => GET,
    "has" => HAS,
    ":+" => PUSH_BACK,
    "+:" => PUSH_FRONT,
    "ins" => INSERT,
    "++" => APPEND,
    ":-" => POP_BACK,
    "-:" => POP_FRONT,
    "rmv" => REMOVE_INDEX,
    ":~" => MOVE_BACK,
    "~:" => MOVE_FRONT,
    "mov" => MOVE_INDEX,
    "del" => DELETE,

    "@" => AT,
    "tk" => TAKE,
    "dp" => DROP,
    "chunk" => CHUNK,
    "window" => WINDOW,
    "divvy" => DIVVY,
    "flat" => FLAT,
    "rflat" => FLAT_REC,
    "sort" => SORT,
    "enum" => ENUM,
    "pairs" => PAIRS,
    "*zip" => MULTI_ZIP,
    "*cprod" => MULTI_CPROD,

    "map" => MAP,
    "@map" => MAP_ATOM,
    "mapf" => MAPF,
    "fltr" => FILTER,
    "?tk" => TAKE_WHILE,
    "?dp" => DROP_WHILE,
    "zip" => ZIP,
    "@zip" => ZIP_ATOM,
    "fold" => FOLD,
    "fold1" => FOLD1,
    "scan" => SCAN,
    "/sort" => SORT_BY,
    "#sort" => SORT_BY_KEY,
    "find" => FIND,
    "any" => ANY,
    "all" => ALL,
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
    env.push(EuType::seq((0..).map(EuType::ibig).map(Ok)));
    Ok(())
};

const STACK: EuDef = |env| {
    env.push(EuType::Vec(env.stack.clone()));
    Ok(())
};

const DUP: EuDef = |env| {
    env.push(env.last()?.clone());
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
    let a0 = env.pop()?.try_isize()?;
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
    let a0 = env.pop()?.try_isize()?;
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
    let a0 = env.pop()?.try_isize()?;
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
    let a0 = env.pop()?.try_isize()?;
    let t = env.stack.remove(env.iflip(a0)?);
    env.push(t);
    Ok(())
};

const ROLL_: EuDef = |env| {
    env.check_nargs(2)?;
    let a0 = env.stack.pop().unwrap().try_isize()?;
    let i = env.iflip(a0)?;
    let t = env.stack.pop().unwrap();
    env.stack.insert(i, t);
    Ok(())
};

const PAIR: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.stack.push(EuType::vec([a0, a1]));
    Ok(())
};

const WRAP: EuDef = |env| {
    let a0 = EuType::Vec(mem::take(&mut env.stack));
    env.stack.push(a0);
    Ok(())
};

const WRAP_: EuDef = |env| {
    let a0 = env.pop()?.to_vec()?;
    env.stack.extend(a0);
    Ok(())
};

const USURP: EuDef = |env| {
    env.stack = env.pop()?.to_vec()?;
    Ok(())
};

const SUB_STACK: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap().to_expr()?;
    let a0 = env.stack.pop().unwrap().to_vec()?;
    env.push(EuType::Vec(EuEnv::apply(a1, &a0, env.scope.clone())?.stack));
    Ok(())
};

const DIP: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap().to_expr()?;
    let a0 = env.stack.pop().unwrap();
    env.stack = EuEnv::apply(a1, &env.stack, env.scope.clone())?.stack;
    env.push(a0);
    Ok(())
};

const READ: EuDef = |env| {
    env.push(EuType::res_str(
        io::read_to_string(io::stdin())
            .map(EuType::str)
            .map_err(|e| anyhow!(e).into()),
    ));
    Ok(())
};

const READLN: EuDef = |env| {
    let mut res = String::new();
    env.push(EuType::res_str(
        io::stdin()
            .read_line(&mut res)
            .map(|_| EuType::str(res))
            .map_err(|e| anyhow!(e).into()),
    ));
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

const EVAL_RES: EuDef = |env| {
    let a0 = env.pop()?.to_expr()?;
    env.push(EuType::res_str(EuEnv::apply_n_1(
        a0,
        &env.stack,
        env.scope.clone(),
    )));
    Ok(())
};

const TO_BOOL: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::Bool(a0.into()));
    Ok(())
};

#[crabtime::function]
fn gen_def_to_num() {
    let types = ["I32", "I64", "F32", "F64", "IBig"];
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
    let ts = EuType::Vec(mem::take(&mut env.stack));
    env.push(ts);
    Ok(())
};

const EVAL_VEC: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.eval_to_vec(env.scope.clone())?);
    Ok(())
};

const TO_MAP: EuDef = |env| {
    let a0 = env.pop()?.to_map()?;
    env.push(EuType::Map(a0));
    Ok(())
};

const WRAP_MAP: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::map_([(EuType::I64(0), a0)]));
    Ok(())
};

const ALL_MAP: EuDef = |env| {
    let kvs = EuType::Vec(mem::take(&mut env.stack)).to_map()?;
    env.push(EuType::Map(kvs));
    Ok(())
};

const EVAL_MAP: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.eval_to_map(env.scope.clone())?);
    Ok(())
};

const TO_SET: EuDef = |env| {
    let a0 = env.pop()?.to_set()?;
    env.push(EuType::Set(a0));
    Ok(())
};

const WRAP_SET: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::set([a0]));
    Ok(())
};

const ALL_SET: EuDef = |env| {
    let ts = EuType::Vec(mem::take(&mut env.stack)).to_set()?;
    env.push(EuType::Set(ts));
    Ok(())
};

const EVAL_SET: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.eval_to_set(env.scope.clone())?);
    Ok(())
};

const TO_EXPR: EuDef = |env| {
    let a0 = env.pop()?.to_expr();
    env.push(EuType::res_str(a0.map(EuType::expr)));
    Ok(())
};

const WRAP_EXPR: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::expr([a0.into()]));
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

const FOLD_: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.unfold_env(a1, env.scope.clone())?);
    Ok(())
};

const RPT: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::Seq(a0.repeat()));
    Ok(())
};

const CYC: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(EuType::Seq(a0.cycle()));
    Ok(())
};

const EVAL: EuDef = |env| env.pop()?.for_rec(&mut |f| env.eval_iter(f));

const TAP: EuDef = |env| {
    env.pop()?.for_rec(&mut |f| {
        EuEnv::apply(f, &env.stack, env.scope.clone())?;
        Ok(())
    })
};

const AND_EVAL: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap().into();
    if a0 {
        a1.for_rec(&mut |f| env.eval_iter(f))
    } else {
        Ok(())
    }
};

const OR_EVAL: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap().into();
    if a0 {
        Ok(())
    } else {
        a1.for_rec(&mut |f| env.eval_iter(f))
    }
};

const IF_EVAL: EuDef = |env| {
    env.check_nargs(3)?;
    let a2 = env.stack.pop().unwrap();
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap().into();
    if a0 { a1 } else { a2 }.for_rec(&mut |f| env.eval_iter(f))
};

const BIND_ARGS: EuDef = |env| {
    let a0 = env.pop()?;
    env.bind_args(a0.into())
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
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
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
                let a1 = env.stack.pop().unwrap();
                let a0 = env.stack.pop().unwrap();
                env.push(EuType::Bool(a0 {{op}} a1));
                Ok(())
            };
        }
    }
}

gen_fn_cmp_binops!();

const NEG: EuDef = |env| {
    let a0 = env.pop()?;
    env.push((-a0)?);
    Ok(())
};

#[crabtime::function]
fn gen_fn_math_binops() {
    for name in ["ADD", "SUB", "MUL", "DIV", "REM", "POW"] {
        let op = name.to_lowercase();
        crabtime::output! {
            const {{name}}: EuDef = |env| {
                env.check_nargs(2)?;
                let a1 = env.stack.pop().unwrap();
                let a0 = env.stack.pop().unwrap();
                env.push(a0.{{op}}(a1)?);
                Ok(())
            };
        }
    }
}

gen_fn_math_binops!();

const GET: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(EuType::opt(a0.get(a1)?));
    Ok(())
};

const HAS: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(EuType::Bool(a0.has(&a1)));
    Ok(())
};

const PUSH_BACK: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.push_back(a1)?);
    Ok(())
};

const PUSH_FRONT: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.push_front(a1)?);
    Ok(())
};

const INSERT: EuDef = |env| {
    env.check_nargs(3)?;
    let a2 = env.stack.pop().unwrap();
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a2.vecz1(|n| a0.insert(n.try_isize()?, a1))?);
    Ok(())
};

const APPEND: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.append(a1)?);
    Ok(())
};

const POP_BACK: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.pop_back()?.1);
    Ok(())
};

const POP_FRONT: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.pop_front()?.1);
    Ok(())
};

const REMOVE_INDEX: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a1.vecz1(|n| a0.remove(n.try_isize()?).map(|(_, ts)| ts))?);
    Ok(())
};

const MOVE_BACK: EuDef = |env| {
    let (t, ts) = env.pop()?.pop_back()?;
    env.push(ts);
    env.push(EuType::opt(t));
    Ok(())
};

const MOVE_FRONT: EuDef = |env| {
    let (t, ts) = env.pop()?.pop_front()?;
    env.push(ts);
    env.push(EuType::opt(t));
    Ok(())
};

const MOVE_INDEX: EuDef = |env| {
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

const DELETE: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.delete(a1));
    Ok(())
};

const AT: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a1.vecz1(|n| a0.at(n.try_isize()?).map(EuType::opt))?);
    Ok(())
};

const TAKE: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a1.vecz1(|n| a0.take(n.try_isize()?))?);
    Ok(())
};

const DROP: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a1.vecz1(|n| a0.drop(n.try_isize()?))?);
    Ok(())
};

const CHUNK: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a1.vecz1(|n| a0.chunk(n.try_isize()?))?);
    Ok(())
};

const WINDOW: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a1.vecz1(|n| a0.window(n.try_usize()?))?);
    Ok(())
};

const DIVVY: EuDef = |env| {
    env.check_nargs(3)?;
    let a2 = env.stack.pop().unwrap();
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a1.vecz2(a2, |n, o| a0.divvy(n.try_usize()?, o.try_isize()?))?);
    Ok(())
};

const SORT: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.sorted()?);
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

const ENUM: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.enumerate());
    Ok(())
};

const PAIRS: EuDef = |env| {
    let a0 = env.pop()?;
    env.push(a0.pairs());
    Ok(())
};

const MULTI_ZIP: EuDef = |env| {
    let a0 = env.pop()?.to_vec()?;
    env.push(EuType::seq(EuType::multi_zip(a0)));
    Ok(())
};

const MULTI_CPROD: EuDef = |env| {
    let a0 = env.pop()?.to_vec()?;
    env.push(EuType::seq(EuType::multi_cartesian_product(a0)));
    Ok(())
};

const MAP: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.map_env(a1, env.scope.clone())?);
    Ok(())
};

const MAP_ATOM: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.map_atom_env(a1, env.scope.clone())?);
    Ok(())
};

const MAPF: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.flat_map_env(a1, env.scope.clone())?);
    Ok(())
};

const FILTER: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.filter_env(a1, env.scope.clone())?);
    Ok(())
};

const TAKE_WHILE: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.take_while_env(a1, env.scope.clone())?);
    Ok(())
};

const DROP_WHILE: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.drop_while_env(a1, env.scope.clone())?);
    Ok(())
};

const ZIP: EuDef = |env| {
    env.check_nargs(3)?;
    let a2 = env.stack.pop().unwrap();
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.zip_env(a1, a2, env.scope.clone())?);
    Ok(())
};

const ZIP_ATOM: EuDef = |env| {
    env.check_nargs(2)?;
    let a2 = env.stack.pop().unwrap();
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.zip_atom_env(a1, a2, env.scope.clone())?);
    Ok(())
};

const FOLD: EuDef = |env| {
    env.check_nargs(3)?;
    let a2 = env.stack.pop().unwrap();
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.fold_env(a1, a2, env.scope.clone())?);
    Ok(())
};

const FOLD1: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.fold1_env(a1, env.scope.clone())?);
    Ok(())
};

const SCAN: EuDef = |env| {
    env.check_nargs(3)?;
    let a2 = env.stack.pop().unwrap();
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.scan_env(a1, a2, env.scope.clone())?);
    Ok(())
};

const SORT_BY: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.sorted_by_env(a1, env.scope.clone())?);
    Ok(())
};

const SORT_BY_KEY: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.sorted_by_key_env(a1, env.scope.clone())?);
    Ok(())
};

const FIND: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.find_env(a1, env.scope.clone())?);
    Ok(())
};

const ANY: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.any_env(a1, env.scope.clone())?);
    Ok(())
};

const ALL: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(a0.all_env(a1, env.scope.clone())?);
    Ok(())
};
