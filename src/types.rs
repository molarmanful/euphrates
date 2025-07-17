use derive_more::{
    Debug,
    IsVariant,
};
use hipstr::HipStr;

#[derive(Debug, PartialEq, Clone, IsVariant)]
pub enum EuType<'eu> {
    Bool(bool),
    Isize(isize),
    Usize(usize),
    I32(i32),
    U32(u32),
    F32(f32),
    I64(i64),
    U64(u64),
    F64(f64),
    Char(char),
    Str(HipStr<'eu>),
    Word(HipStr<'eu>),
    Opt(Option<Box<EuType<'eu>>>),
    Res(Result<Box<EuType<'eu>>, Box<EuType<'eu>>>),
    Vec(Vec<EuType<'eu>>),
    Expr(Vec<EuType<'eu>>),
}
