use phf::phf_map;

use super::{
    EuDef,
    EuFnMeta,
};
use crate::types::EuType;

pub const CORE: phf::Map<&'static str, EuDef> = phf_map! {
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
    "some" => SOME,
    "none" => NONE,
};

const DUP: EuDef = (EuFnMeta::nargs(1), |st, _| {
    st.stack.0.push(
        st.stack
            .0
            .last()
            .expect("stack length should be checked")
            .clone(),
    );
    None
});

const DUPS: EuDef = (EuFnMeta::nargs(0), |st, _| {
    st.stack.0.push(EuType::Vec(st.stack.0.clone().into()));
    None
});

const DUPD: EuDef = (EuFnMeta::nargs(2), |st, _| {
    st.stack
        .0
        .insert(st.stack.iflip(1), st.stack.0[st.stack.iflip(1)].clone());
    None
});

const OVER: EuDef = (EuFnMeta::nargs(2), |st, _| {
    st.stack.0.push(st.stack.0[st.stack.iflip(1)].clone());
    None
});

const DDUP: EuDef = (EuFnMeta::nargs(2), |st, w| {
    OVER.1(st, w);
    OVER.1(st, w);
    None
});

const EDUP: EuDef = (EuFnMeta::nargs(3), |st, _| {
    st.stack.0.push(st.stack.0[st.stack.iflip(2)].clone());
    st.stack.0.push(st.stack.0[st.stack.iflip(2)].clone());
    st.stack.0.push(st.stack.0[st.stack.iflip(2)].clone());
    None
});

const POP: EuDef = (EuFnMeta::nargs(1), |st, _| {
    st.stack.0.pop();
    None
});

const CLR: EuDef = (EuFnMeta::new(), |st, _| {
    st.stack.0.clear();
    None
});

const NIP: EuDef = (EuFnMeta::nargs(2), |st, _| {
    st.stack.0.swap_remove(st.stack.iflip(1));
    None
});

const PPOP: EuDef = (EuFnMeta::nargs(2), |st, _| {
    st.stack.0.truncate(st.stack.iflip(1));
    None
});

const QPOP: EuDef = (EuFnMeta::nargs(3), |st, _| {
    st.stack.0.truncate(st.stack.iflip(2));
    None
});

const SWAP: EuDef = (EuFnMeta::nargs(2), |st, _| {
    let a = st.stack.iflip(0);
    let b = st.stack.iflip(1);
    st.stack.0.swap(a, b);
    None
});

const REV: EuDef = (EuFnMeta::new(), |st, _| {
    st.stack.0.reverse();
    None
});

const SWAPD: EuDef = (EuFnMeta::nargs(3), |st, _| {
    let a = st.stack.iflip(1);
    let b = st.stack.iflip(2);
    st.stack.0.swap(a, b);
    None
});

const TUCK: EuDef = (EuFnMeta::nargs(2), |st, _| {
    st.stack.0.insert(
        st.stack.iflip(1),
        st.stack
            .0
            .last()
            .expect("stack length should be checked")
            .clone(),
    );
    None
});

const ROT: EuDef = (EuFnMeta::nargs(3), |st, _| {
    let x = st.stack.0.remove(st.stack.iflip(2));
    st.stack.0.push(x);
    None
});

const ROT_: EuDef = (EuFnMeta::nargs(3), |st, _| {
    let x = st.stack.0.pop().unwrap();
    st.stack.0.insert(st.stack.iflip(1), x);
    None
});

const SOME: EuDef = (EuFnMeta::nargs(1), |st, _| {
    let x = Box::new(st.stack.0.pop().unwrap());
    st.stack.0.push(EuType::Opt(Some(x).into()));
    None
});

const NONE: EuDef = (EuFnMeta::new(), |st, _| {
    st.stack.0.push(EuType::Opt(None.into()));
    None
});
