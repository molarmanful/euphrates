use dashu_int::IBig;
use derive_more::{
    Debug,
    Display,
    IsVariant,
};
use ecow::EcoVec;
use hipstr::LocalHipStr;
use itertools::Itertools;
use num_traits::Signed;
use ordered_float::OrderedFloat;

use crate::{
    fns::bind,
    types::EuType,
};

#[derive(Debug, Display, Hash, Clone, IsVariant, PartialEq, Eq, PartialOrd, Ord)]
pub enum EuBind<'eu> {
    #[debug("{_0}")]
    Word(LocalHipStr<'eu>),
    #[debug("${_0}({})", _1.iter().map(|t| format!("{t:?}")).join(" "))]
    #[display("{}", _1.iter().join(" "))]
    Tag(LocalHipStr<'eu>, EcoVec<Self>),
    #[debug("({})", _0.iter().map(|t| format!("{t:?}")).join(" "))]
    #[display("{}", _0.iter().join(" "))]
    Union(EcoVec<Self>),
    #[debug("({_0:?}\\{_1:?})")]
    #[display("{_0}{_1}")]
    Bind(Box<Self>, Box<Self>),

    #[debug("{}", if *_0 { "True" } else { "False" })]
    Bool(bool),
    #[debug("{_0:?}i32")]
    I32(i32),
    #[debug("{_0:?}i64")]
    I64(i64),
    #[debug("{_0:?}")]
    IBig(IBig),
    #[debug("{}", if _0.is_infinite() { format!("{}Inf32", if _0.is_negative() {"-"} else {""}) } else { format!("{_0:?}") })]
    F64(OrderedFloat<f64>),
    #[debug("{_0:?}")]
    Char(char),

    #[debug("{_0:?}")]
    Str(LocalHipStr<'eu>),

    #[debug("\\[{}]", _0.iter().map(|t| format!("{t:?}")).join(" "))]
    #[display("{}", _0.iter().join(" "))]
    Vecz(EcoVec<Self>),

    #[debug("{{{}}}", _0.iter().map(|t| format!("{t:?}")).join(" "))]
    #[display("{}", _0.iter().join(" "))]
    Map(EcoVec<Self>),
}

impl<'eu> EuBind<'eu> {
    #[inline]
    pub fn word(s: impl Into<LocalHipStr<'eu>>) -> Self {
        Self::Word(s.into())
    }

    #[inline]
    pub fn tag(w: impl Into<LocalHipStr<'eu>>, bs: impl Into<EcoVec<Self>>) -> Self {
        Self::Tag(w.into(), bs.into())
    }

    #[inline]
    pub fn union(bs: impl Into<EcoVec<Self>>) -> Self {
        Self::Union(bs.into())
    }

    #[inline]
    pub fn bind(b0: impl Into<Self>, b1: impl Into<Self>) -> Self {
        Self::Bind(Box::new(b0.into()), Box::new(b1.into()))
    }

    #[inline]
    pub fn i32(n: impl Into<i32>) -> Self {
        Self::I32(n.into())
    }

    #[inline]
    pub fn i64(n: impl Into<i64>) -> Self {
        Self::I64(n.into())
    }

    #[inline]
    pub fn ibig(n: impl Into<IBig>) -> Self {
        Self::IBig(n.into())
    }

    #[inline]
    pub fn f64(n: impl Into<OrderedFloat<f64>>) -> Self {
        Self::F64(n.into())
    }

    #[inline]
    pub fn str(s: impl Into<LocalHipStr<'eu>>) -> Self {
        Self::Str(s.into())
    }

    #[inline]
    pub fn vecz(bs: impl Into<EcoVec<Self>>) -> Self {
        Self::Vecz(bs.into())
    }

    #[inline]
    pub fn map(bs: impl Into<EcoVec<Self>>) -> Self {
        Self::Map(bs.into())
    }

    #[must_use]
    pub fn to_free(self) -> Option<EuType<'eu>> {
        match self {
            EuBind::Bind(b0, b1) => b0.to_free().or_else(|| b1.to_free()),
            EuBind::Tag(w, bs) => (bind::BIND.get(&w)?.free)(bs),
            EuBind::Bool(b) => Some(EuType::Bool(b)),
            EuBind::I32(n) => Some(EuType::I32(n)),
            EuBind::I64(n) => Some(EuType::I64(n)),
            EuBind::IBig(n) => Some(EuType::IBig(n)),
            EuBind::F64(n) => Some(EuType::F64(n)),
            EuBind::Char(c) => Some(EuType::Char(c)),
            EuBind::Str(s) => Some(EuType::Str(s)),
            EuBind::Vecz(bs) => (bind::VECZ.free)(bs),
            EuBind::Map(bs) => (bind::MAP.free)(bs),
            _ => None,
        }
    }
}
