use std::iter;

use anyhow::anyhow;
use dashu_int::IBig;
use derive_more::{
    Debug,
    Display,
    IsVariant,
};
use ecow::{
    EcoVec,
    eco_vec,
};
use hipstr::HipStr;
use itertools::Itertools;
use num_traits::{
    Signed,
    ToPrimitive,
};
use ordered_float::OrderedFloat;
use winnow::Parser;

use super::{
    EuRes,
    EuSeq,
};
use crate::parser::euphrates;

#[derive(Debug, Display, Clone, IsVariant)]
#[display("{_0}")]
pub enum EuType<'eu> {
    #[debug("{}", if *_0 { "True" } else { "False" })]
    Bool(bool),
    #[debug("{_0:?}i32")]
    I32(i32),
    #[debug("{_0:?}i64")]
    I64(i64),
    #[debug("{_0:?}")]
    IBig(IBig),
    #[debug("{}", if _0.is_infinite() { format!("{}Inf32", if _0.is_negative() {"-"} else {""}) } else { format!("{_0:?}") })]
    F32(OrderedFloat<f32>),
    #[debug("{}", if _0.is_infinite() { format!("{}Inf", if _0.is_negative() {"-"} else {""}) } else { format!("{_0:?}") })]
    F64(OrderedFloat<f64>),
    #[debug("{_0:?}")]
    Char(char),

    #[debug("{_0:?}")]
    Str(HipStr<'eu>),
    #[debug("{_0}")]
    Word(HipStr<'eu>),

    #[debug("{}", if let Some(t) = _0 { format!("Some:{t:?}") } else { "None".into() })]
    #[display("{}", if let Some(t) = _0 { t.to_string() } else { "".to_string() })]
    Opt(Option<Box<Self>>),
    #[debug("{}", match _0 { Ok(t) => format!("Ok:{t:?}"), Err(e) => format!("Err:{e:?}") })]
    #[display("{}", match _0 { Ok(t) => t.to_string(), Err(e) => e.to_string() })]
    Res(Result<Box<Self>, Box<Self>>),

    #[debug("Vec:({})", _0.iter().map(|t| format!("{t:?}")).join(" "))]
    #[display("{}", _0.iter().join(""))]
    Vec(EcoVec<Self>),
    #[debug("({})", _0.iter().map(|t| format!("{t:?}")).join(" "))]
    #[display("{}", _0.iter().join(" "))]
    Expr(EcoVec<Self>),
    #[debug("Seq:(...)")]
    #[display("{}", _0.clone().map(Self::res_str).join(""))]
    Seq(EuSeq<'eu>),
}

impl<'eu> EuType<'eu> {
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
    pub fn f32(n: impl Into<OrderedFloat<f32>>) -> Self {
        Self::F32(n.into())
    }

    #[inline]
    pub fn f64(n: impl Into<OrderedFloat<f64>>) -> Self {
        Self::F64(n.into())
    }

    #[inline]
    pub fn char(n: impl Into<char>) -> Self {
        Self::Char(n.into())
    }

    #[inline]
    pub fn str(s: impl Into<HipStr<'eu>>) -> Self {
        Self::Str(s.into())
    }

    #[inline]
    pub fn word(s: impl Into<HipStr<'eu>>) -> Self {
        Self::Word(s.into())
    }

    #[inline]
    pub fn opt(o: Option<Self>) -> Self {
        Self::Opt(o.map(Box::new))
    }

    #[inline]
    pub fn res(r: Result<Self, Self>) -> Self {
        Self::Res(r.map(Box::new).map_err(Box::new))
    }

    #[inline]
    pub fn res_str(r: EuRes<Self>) -> Self {
        Self::res(r.map_err(|s| Self::str(s.to_string())))
    }

    #[inline]
    pub fn vec(ts: impl Into<EcoVec<Self>>) -> Self {
        Self::Vec(ts.into())
    }

    #[inline]
    pub fn expr(ts: impl Into<EcoVec<Self>>) -> Self {
        Self::Expr(ts.into())
    }

    #[inline]
    pub fn seq<I>(it: I) -> Self
    where
        I: Iterator<Item = EuRes<Self>> + Clone + 'eu,
    {
        Self::Seq(Box::new(it))
    }

    pub fn to_vec(self) -> EuRes<EcoVec<Self>> {
        match self {
            Self::Vec(ts) | Self::Expr(ts) => Ok(ts),
            Self::Str(s) => s.chars().map(|t| Ok(Self::Char(t))).collect(),
            Self::Opt(o) => o.into_iter().map(|t| Ok(*t)).collect(),
            Self::Res(r) => r.into_iter().map(|t| Ok(*t)).collect(),
            Self::Seq(it) => it.collect(),
            _ => Ok(eco_vec![self]),
        }
    }

    pub fn to_expr(self) -> EuRes<EcoVec<Self>> {
        match self {
            Self::Expr(ts) => Ok(ts),
            Self::Str(s) => euphrates
                .parse(&s)
                .map_err(|e| anyhow!(e.into_inner()).into()),
            _ => Ok(eco_vec![self]),
        }
    }

    pub fn to_seq(self) -> EuSeq<'eu> {
        match self {
            Self::Seq(it) => it,
            Self::Str(s) => Box::new(
                s.chars()
                    .collect_vec()
                    .into_iter()
                    .map(|t| Ok(Self::Char(t))),
            ),
            Self::Opt(o) => Box::new(o.into_iter().map(|t| Ok(*t))),
            Self::Res(r) => Box::new(r.into_iter().map(|t| Ok(*t))),
            Self::Vec(ts) | Self::Expr(ts) => Box::new(ts.as_slice().to_vec().into_iter().map(Ok)),
            t => Box::new(iter::once(Ok(t))),
        }
    }

    #[inline]
    pub fn is_num(&self) -> bool {
        self.is_i_32() || self.is_i_64() || self.is_i_big() || self.is_f_32() || self.is_f_64()
    }

    #[inline]
    pub fn is_num_like(&self) -> bool {
        self.is_num() || self.is_bool() || self.is_char()
    }

    #[inline]
    pub fn is_num_parse(&self) -> bool {
        self.is_num() || self.is_str()
    }

    #[inline]
    pub fn is_vecz(&self) -> bool {
        self.is_once() || self.is_many()
    }

    #[inline]
    pub fn is_once(&self) -> bool {
        self.is_opt() || self.is_res()
    }

    #[inline]
    pub fn is_many(&self) -> bool {
        self.is_vec() || self.is_seq()
    }
}

#[crabtime::function]
fn gen_type_to_num() {
    let types = ["I32", "I64", "IBig", "F32", "F64"];
    for t0 in types {
        let tl = t0.to_lowercase();
        let tlp = if t0 == "IBig" { t0 } else { &tl };
        let tlq = format!(r#""{tl}""#);
        let arms = types
            .map(|t1| {
                if t1 == t0 {
                    if t0.chars().next() == Some('I') {
                        crabtime::quote! {
                            Self::{{t1}}(n) => Some(n),
                        }
                    } else {
                        crabtime::quote! {
                            Self::{{t1}}(n) => Some(n.0),
                        }
                    }
                } else if t1 == "IBig" && t0.chars().next() == Some('F') {
                    crabtime::quote! {
                        Self::{{t1}}(n) => Some(n.to_{{tl}}().value()),
                    }
                } else if t0 == "IBig" {
                    if t1.chars().next() == Some('I') {
                        crabtime::quote! {
                            Self::{{t1}}(n) => Some(n.into()),
                        }
                    } else {
                        crabtime::quote! {
                            Self::{{t1}}(n) => n.0.try_into().ok(),
                        }
                    }
                } else {
                    crabtime::quote! {
                        Self::{{t1}}(n) => n.to_{{tl}}(),
                    }
                }
            })
            .join("");

        crabtime::output! {
            impl EuType<'_> {
                #[inline]
                pub fn try_{{tl}}(self) -> EuRes<{{tlp}}> {
                    self.to_{{tl}}().ok_or_else(|| anyhow!(concat!({{tlq}}, " conversion failed")).into())
                }

                pub fn to_{{tl}}(self) -> Option<{{tlp}}> {
                    match self {
                        {{arms}}
                        Self::Bool(b) => Some(b.into()),
                        Self::Char(c) => Self::I32(c as i32).to_{{tl}}(),
                        Self::Str(s) => s.parse().ok(),
                        _ => None,
                    }
                }
            }
        }
    }
}

gen_type_to_num!();

#[crabtime::function]
fn gen_type_to_size() {
    let types = ["I32", "I64", "IBig", "F32", "F64"];
    for n in ["isize", "usize"] {
        let nq = format!(r#""{n}""#);
        let arms = types
            .map(|t| {
                crabtime::quote! {
                    Self::{{t}}(n) => n.to_{{n}}(),
                }
            })
            .join("");

        crabtime::output! {
            impl EuType<'_> {
                #[inline]
                pub fn try_{{n}}(self) -> EuRes<{{n}}> {
                    self.to_{{n}}().ok_or_else(|| anyhow!(concat!({{nq}}, " conversion failed")).into())
                }

                pub fn to_{{n}}(self) -> Option<{{n}}> {
                    match self {
                        {{arms}}
                        Self::Bool(b) => Some(b.into()),
                        Self::Char(c) => (c as u32).to_{{n}}(),
                        Self::Str(s) => s.parse().ok(),
                        _ => None
                    }
                }
            }
        }
    }
}

gen_type_to_size!();

#[crabtime::function]
fn gen_type_to_bool() {
    let types = ["I32", "I64", "F32", "F64"];
    let arms = types
        .map(|t| {
            let n = t.to_lowercase();
            crabtime::quote! {
                EuType::{{t}}(n) => n != 0 as {{n}},
            }
        })
        .join("");

    crabtime::output! {
        impl From<EuType<'_>> for bool {
            fn from(value: EuType) -> Self {
                match value {
                    EuType::Bool(b) => b,
                    {{arms}}
                    EuType::IBig(n) => !n.is_zero(),
                    EuType::Char(c) => c != '\0',
                    EuType::Str(s) => !s.is_empty(),
                    EuType::Word(_) => true,
                    EuType::Opt(o) => o.is_some(),
                    EuType::Res(r) => r.is_ok(),
                    EuType::Vec(ts) | EuType::Expr(ts) => !ts.is_empty(),
                    EuType::Seq(it) => Iterator::peekable(it).peek().is_some(),
                }
            }
        }
    }
}

gen_type_to_bool!();
