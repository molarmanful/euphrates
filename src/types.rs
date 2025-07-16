use derive_more::{
    Debug,
    Display,
    From,
    IntoIterator,
    IsVariant,
    TryInto,
};
use hipstr::HipStr;

#[derive(Debug, PartialEq, Clone, From, TryInto, IsVariant)]
pub enum EuType<'eu> {
    Bool(EuBool),
    Isize(EuIsize),
    Usize(EuUsize),
    I32(EuI32),
    U32(EuU32),
    F32(EuF32),
    I64(EuI64),
    U64(EuU64),
    F64(EuF64),
    Char(EuChar),
    Str(EuStr<'eu>),
    Word(EuWord<'eu>),
    Opt(EuOpt<'eu>),
    Res(EuRes<'eu>),
    Vec(EuVec<'eu>),
}

#[derive(Debug, Display, PartialEq, Clone, From, Default)]
#[from(forward)]
pub struct EuBool(pub bool);

#[derive(Debug, Display, PartialEq, Clone, From, Default)]
#[from(forward)]
pub struct EuIsize(pub isize);

#[derive(Debug, Display, PartialEq, Clone, From, Default)]
#[from(forward)]
pub struct EuUsize(pub usize);

#[derive(Debug, Display, PartialEq, Clone, From, Default)]
#[from(forward)]
pub struct EuI32(pub i32);

#[derive(Debug, Display, PartialEq, Clone, From, Default)]
#[from(forward)]
pub struct EuU32(pub u32);

#[derive(Debug, Display, PartialEq, Clone, From, Default)]
#[from(forward)]
pub struct EuF32(pub f32);

#[derive(Debug, Display, PartialEq, Clone, From, Default)]
#[from(forward)]
pub struct EuI64(pub i64);

#[derive(Debug, Display, PartialEq, Clone, From, Default)]
#[from(forward)]
pub struct EuU64(pub u64);

#[derive(Debug, Display, PartialEq, Clone, From, Default)]
#[from(forward)]
pub struct EuF64(pub f64);

#[derive(Debug, Display, PartialEq, Clone, From, Default)]
#[from(forward)]
pub struct EuChar(pub char);

#[derive(Debug, Display, PartialEq, Clone, From, Default)]
#[from(forward)]
pub struct EuStr<'s>(pub HipStr<'s>);

#[derive(Debug, Display, PartialEq, Clone, From, Default)]
#[from(forward)]
pub struct EuWord<'s>(pub HipStr<'s>);

#[derive(Debug, PartialEq, Clone, From, Default)]
#[from(forward)]
pub struct EuOpt<'eu>(pub Option<Box<EuType<'eu>>>);

#[derive(Debug, PartialEq, Clone, From)]
pub struct EuRes<'eu>(pub Result<Box<EuType<'eu>>, Box<EuType<'eu>>>);

#[derive(Debug, PartialEq, Clone, From, IntoIterator, Default)]
#[from(forward)]
#[into_iterator(owned, ref, ref_mut)]
pub struct EuVec<'eu>(pub Vec<EuType<'eu>>);
