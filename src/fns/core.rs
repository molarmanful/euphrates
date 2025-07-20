use ecow::eco_vec;
use phf::phf_map;

use super::{
    EuDef,
    META,
};
use crate::types::EuType;

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
    "swapd" => SWAP,
    "tuck" => TUCK,
    "rot" => ROT,
    "rot_" => ROT_,
    "Some" => SOME,
    "None" => NONE,
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
};

const DUP: EuDef = (META.nargs(1), |env, _| {
    env.x.stack.push(env.x.stack.last().unwrap().clone());
    None
});

const DUPS: EuDef = (META.nargs(0), |env, _| {
    env.x.stack.push(EuType::Vec(env.x.stack.clone()));
    None
});

const DUPD: EuDef = (META.nargs(2), |env, _| {
    env.x
        .stack
        .insert(env.x.iflip(1), env.x.stack[env.x.iflip(1)].clone());
    None
});

const OVER: EuDef = (META.nargs(2), |env, _| {
    env.x.stack.push(env.x.stack[env.x.iflip(1)].clone());
    None
});

const DDUP: EuDef = (META.nargs(2), |env, w| {
    OVER.1(env, w);
    OVER.1(env, w);
    None
});

const EDUP: EuDef = (META.nargs(3), |env, _| {
    env.x.stack.push(env.x.stack[env.x.iflip(2)].clone());
    env.x.stack.push(env.x.stack[env.x.iflip(2)].clone());
    env.x.stack.push(env.x.stack[env.x.iflip(2)].clone());
    None
});

const POP: EuDef = (META.nargs(1), |env, _| {
    env.x.stack.pop();
    None
});

const CLR: EuDef = (META, |env, _| {
    env.x.stack.clear();
    None
});

const NIP: EuDef = (META.nargs(2), |env, _| {
    env.x.stack.remove(env.x.iflip(1));
    None
});

const PPOP: EuDef = (META.nargs(2), |env, _| {
    env.x.stack.truncate(env.x.iflip(1));
    None
});

const QPOP: EuDef = (META.nargs(3), |env, _| {
    env.x.stack.truncate(env.x.iflip(2));
    None
});

const SWAP: EuDef = (META.nargs(2), |env, _| {
    let a = env.x.iflip(0);
    env.x.stack.make_mut().swap(a, a - 1);
    None
});

const REV: EuDef = (META, |env, _| {
    env.x.stack.make_mut().reverse();
    None
});

const SWAPD: EuDef = (META.nargs(3), |env, _| {
    let a = env.x.iflip(1);
    env.x.stack.make_mut().swap(a, a - 1);
    None
});

const TUCK: EuDef = (META.nargs(2), |env, _| {
    env.x
        .stack
        .insert(env.x.iflip(1), env.x.stack.last().unwrap().clone());
    None
});

const ROT: EuDef = (META.nargs(3), |env, _| {
    let a0 = env.x.stack.remove(env.x.iflip(2));
    env.x.stack.push(a0);
    None
});

const ROT_: EuDef = (META.nargs(3), |env, _| {
    let a0 = env.x.stack.pop().unwrap();
    env.x.stack.insert(env.x.iflip(1), a0);
    None
});

const SOME: EuDef = (META.nargs(1), |env, _| {
    let a0 = Box::new(env.x.stack.pop().unwrap());
    env.x.stack.push(EuType::Opt(Some(a0)));
    None
});

const NONE: EuDef = (META, |env, _| {
    env.x.stack.push(EuType::Opt(None));
    None
});

const OK: EuDef = (META.nargs(1), |env, _| {
    let a0 = env.x.stack.pop().unwrap();
    env.x.stack.push(EuType::Res(Ok(Box::new(a0))));
    None
});

const ERR: EuDef = (META.nargs(1), |env, _| {
    let a0 = env.x.stack.pop().unwrap();
    env.x.stack.push(EuType::Res(Err(Box::new(a0))));
    None
});

const TO_BOOL: EuDef = (META.nargs(1), |env, _| {
    let a0 = env.x.stack.pop().unwrap();
    env.x.stack.push(EuType::Bool(a0.into()));
    None
});

#[crabtime::function]
fn gen_def_to_num() {
    let types = [
        "Isize", "Usize", "I32", "U32", "F32", "I64", "U64", "F64", "I128", "U128",
    ];
    for &t in &types {
        let n = t.to_lowercase();
        let n_up = t.to_uppercase();
        crabtime::output! {
            const TO_{{n_up}}: EuDef = (META.nargs(1), |env, _| {
                let x = env.x.stack.pop().unwrap();
                env.x.stack
                    .push(EuType::Opt(x.to_{{n}}().map(EuType::{{t}}).map(Box::new)));
                None
            });
        }
    }
}

gen_def_to_num!();

const WRAP_VEC: EuDef = (META.nargs(1), |env, _| {
    let a0 = env.x.stack.pop().unwrap();
    env.x.stack.push(EuType::Vec(eco_vec![a0]));
    None
});

const TO_VEC: EuDef = (META.nargs(1), |env, _| {
    let a0 = env.x.stack.pop().unwrap();
    env.x.stack.push(EuType::Vec(a0.to_vec()));
    None
});

const WRAP_EXPR: EuDef = (META.nargs(1), |env, _| {
    let a0 = env.x.stack.pop().unwrap();
    env.x.stack.push(EuType::Expr(eco_vec![a0]));
    None
});

const TO_EXPR: EuDef = (META.nargs(1), |env, _| {
    let a0 = env.x.stack.pop().unwrap().to_expr();
    env.x.stack.push(EuType::Res(
        a0.map(|ts| Box::new(EuType::Vec(ts)))
            .map_err(|e| Box::new(EuType::Str(e.to_string().into()))),
    ));
    None
});

const EVAL: EuDef = (META.nargs(1), |env, _| {
    let a0 = env.x.stack.pop().unwrap().to_expr();
    match a0 {
        Ok(ts) => env.eval_iter(ts),
        Err(e) => Some(e),
    }
});
