use std::collections::VecDeque;

use hipstr::HipStr;

#[derive(Debug, PartialEq, Clone)]
pub enum EuType<'eu> {
    Bool(bool),
    ISize(isize),
    USize(usize),
    I32(i32),
    U32(u32),
    F32(f64),
    I64(i64),
    U64(u64),
    F64(f64),
    Char(char),
    Str(HipStr<'eu>),
    Word(HipStr<'eu>),
    Opt(EuOpt<'eu>),
    Arr(EuVec<'eu>),
    Fn(EuFn<'eu>),
}

pub type EuOpt<'eu> = Option<Box<EuType<'eu>>>;

pub type EuVec<'eu> = Vec<EuType<'eu>>;

pub type EuFn<'eu> = VecDeque<EuType<'eu>>;
