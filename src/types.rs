use std::sync::Arc;

use derive_more::{
    Debug,
    From,
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
    Arr(EuVec<'eu>),
    Fn(EuFn<'eu>),
}

#[derive(Debug, PartialEq, Clone, From)]
#[from(forward)]
pub struct EuBool(pub bool);

#[derive(Debug, PartialEq, Clone, From)]
#[from(forward)]
pub struct EuIsize(pub isize);

#[derive(Debug, PartialEq, Clone, From)]
#[from(forward)]
pub struct EuUsize(pub usize);

#[derive(Debug, PartialEq, Clone, From)]
#[from(forward)]
pub struct EuI32(pub i32);

#[derive(Debug, PartialEq, Clone, From)]
#[from(forward)]
pub struct EuU32(pub u32);

#[derive(Debug, PartialEq, Clone, From)]
#[from(forward)]
pub struct EuF32(pub f32);

#[derive(Debug, PartialEq, Clone, From)]
#[from(forward)]
pub struct EuI64(pub i64);

#[derive(Debug, PartialEq, Clone, From)]
#[from(forward)]
pub struct EuU64(pub u64);

#[derive(Debug, PartialEq, Clone, From)]
#[from(forward)]
pub struct EuF64(pub f64);

#[derive(Debug, PartialEq, Clone, From)]
#[from(forward)]
pub struct EuChar(pub char);

#[derive(Debug, PartialEq, Clone, From)]
#[from(forward)]
pub struct EuStr<'s>(pub HipStr<'s>);

#[derive(Debug, PartialEq, Clone, From)]
#[from(forward)]
pub struct EuWord<'s>(pub HipStr<'s>);

#[derive(Debug, PartialEq, Clone, From)]
#[from(forward)]
pub struct EuOpt<'eu>(pub Option<Box<EuType<'eu>>>);

#[derive(Debug, PartialEq, Clone, From)]
#[from(forward)]
pub struct EuVec<'eu>(pub Vec<EuType<'eu>>);

#[derive(Debug, Clone, From)]
#[from(forward)]
#[debug("<iterator>")]
pub struct EuFn<'eu>(pub Arc<dyn Iterator<Item = EuType<'eu>>>);

impl PartialEq for EuFn<'_> {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
