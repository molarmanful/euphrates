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
};

const DUP: EuDef = (META.nargs(1), |st, _| {
    st.stack.push(
        st.stack
            .last()
            .expect("stack length should be checked")
            .clone(),
    );
    None
});

const DUPS: EuDef = (META.nargs(0), |st, _| {
    st.stack.push(EuType::Vec(st.stack.clone()));
    None
});

const DUPD: EuDef = (META.nargs(2), |st, _| {
    st.stack.insert(st.iflip(1), st.stack[st.iflip(1)].clone());
    None
});

const OVER: EuDef = (META.nargs(2), |st, _| {
    st.stack.push(st.stack[st.iflip(1)].clone());
    None
});

const DDUP: EuDef = (META.nargs(2), |st, w| {
    OVER.1(st, w);
    OVER.1(st, w);
    None
});

const EDUP: EuDef = (META.nargs(3), |st, _| {
    st.stack.push(st.stack[st.iflip(2)].clone());
    st.stack.push(st.stack[st.iflip(2)].clone());
    st.stack.push(st.stack[st.iflip(2)].clone());
    None
});

const POP: EuDef = (META.nargs(1), |st, _| {
    st.stack.pop();
    None
});

const CLR: EuDef = (META, |st, _| {
    st.stack.clear();
    None
});

const NIP: EuDef = (META.nargs(2), |st, _| {
    st.stack.swap_remove(st.iflip(1));
    None
});

const PPOP: EuDef = (META.nargs(2), |st, _| {
    st.stack.truncate(st.iflip(1));
    None
});

const QPOP: EuDef = (META.nargs(3), |st, _| {
    st.stack.truncate(st.iflip(2));
    None
});

const SWAP: EuDef = (META.nargs(2), |st, _| {
    let a = st.iflip(0);
    let b = st.iflip(1);
    st.stack.swap(a, b);
    None
});

const REV: EuDef = (META, |st, _| {
    st.stack.reverse();
    None
});

const SWAPD: EuDef = (META.nargs(3), |st, _| {
    let a = st.iflip(1);
    let b = st.iflip(2);
    st.stack.swap(a, b);
    None
});

const TUCK: EuDef = (META.nargs(2), |st, _| {
    st.stack.insert(
        st.iflip(1),
        st.stack
            .last()
            .expect("stack length should be checked")
            .clone(),
    );
    None
});

const ROT: EuDef = (META.nargs(3), |st, _| {
    let x = st.stack.remove(st.iflip(2));
    st.stack.push(x);
    None
});

const ROT_: EuDef = (META.nargs(3), |st, _| {
    let x = st.stack.pop().unwrap();
    st.stack.insert(st.iflip(1), x);
    None
});

const SOME: EuDef = (META.nargs(1), |st, _| {
    let x = Box::new(st.stack.pop().unwrap());
    st.stack.push(EuType::Opt(Some(x)));
    None
});

const NONE: EuDef = (META, |st, _| {
    st.stack.push(EuType::Opt(None));
    None
});

const OK: EuDef = (META.nargs(1), |st, _| {
    let x = st.stack.pop().unwrap();
    st.stack.push(EuType::Res(Ok(Box::new(x))));
    None
});

const ERR: EuDef = (META.nargs(1), |st, _| {
    let x = st.stack.pop().unwrap();
    st.stack.push(EuType::Res(Err(Box::new(x))));
    None
});

const TO_BOOL: EuDef = (META.nargs(1), |st, _| {
    let x = st.stack.pop().unwrap();
    st.stack.push(EuType::Bool(x.into()));
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
            const TO_{{n_up}}: EuDef = (META.nargs(1), |st, _| {
                let x = st.stack.pop().unwrap();
                st.stack
                    .push(EuType::Opt(x.to_{{n}}().map(EuType::{{t}}).map(Box::new)));
                None
            });
        }
    }
}

gen_def_to_num!();
