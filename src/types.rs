use std::{
    iter,
    mem,
    ops::{
        Add,
        Div,
        Mul,
        Neg,
        Sub,
    },
    sync::{
        Arc,
        Mutex,
    },
};

use anyhow::anyhow;
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
use num_traits::ToPrimitive;
use winnow::Parser;

use crate::{
    parser::euphrates,
    utils::swap_errors,
};

#[derive(Debug, Display, Clone, IsVariant)]
#[display("{_0}")]
pub enum EuType<'eu> {
    #[debug("{}", if *_0 { "True" } else { "False" })]
    Bool(bool),
    #[debug("{_0:?}")]
    I32(i32),
    #[debug("{_0:?}i64")]
    I64(i64),
    #[debug("{_0:?}f32")]
    F32(f32),
    #[debug("{_0:?}")]
    F64(f64),
    #[debug("{_0:?}")]
    Char(char),

    #[debug("{_0:?}")]
    Str(HipStr<'eu>),
    #[debug("{_0}")]
    Word(HipStr<'eu>),

    #[debug("{}", if let Some(t) = _0 { format!("Some:{t:?}") } else { "None".into() })]
    #[display("{}", if let Some(t) = _0 { t.to_string() } else { "".to_string() })]
    Opt(Option<Box<EuType<'eu>>>),
    #[debug("{}", match _0 { Ok(t) => format!("Ok:{t:?}"), Err(e) => format!("Err:{e:?}") })]
    #[display("{}", match _0 { Ok(t) => t.to_string(), Err(e) => e.to_string() })]
    Res(Result<Box<EuType<'eu>>, Box<EuType<'eu>>>),

    #[debug("Vec:({})", _0.iter().map(|t| format!("{t:?}")).join(" "))]
    #[display("{}", _0.iter().join(""))]
    Vec(EcoVec<EuType<'eu>>),
    #[debug("({})", _0.iter().map(|t| format!("{t:?}")).join(" "))]
    #[display("{}", _0.iter().join(" "))]
    Expr(EcoVec<EuType<'eu>>),
    #[debug("Seq:(...)")]
    #[display("{}", Self::take_iter(&mut _0.lock().unwrap()).map(Self::res_str).join(""))]
    Seq(EuSeq<'eu>),
}

type EuSeq<'eu> = Arc<Mutex<EuResIter<'eu>>>;

type EuResIter<'eu> = EuIter<'eu, anyhow::Result<EuType<'eu>>>;

pub type EuIter<'eu, T = EuType<'eu>> = Box<dyn Iterator<Item = T> + Send + Sync + 'eu>;

impl<'eu> EuType<'eu> {
    #[inline]
    pub fn str(s: impl Into<HipStr<'eu>>) -> Self {
        Self::Str(s.into())
    }

    #[inline]
    pub fn word(s: impl Into<HipStr<'eu>>) -> Self {
        Self::Word(s.into())
    }

    #[inline]
    pub fn opt(o: Option<EuType<'eu>>) -> Self {
        Self::Opt(o.map(Box::new))
    }

    #[inline]
    pub fn res(r: Result<Self, Self>) -> Self {
        Self::Res(r.map(Box::new).map_err(Box::new))
    }

    #[inline]
    pub fn res_str(r: anyhow::Result<Self>) -> Self {
        Self::res(r.map_err(|s| Self::str(s.to_string())))
    }

    #[inline]
    pub fn vec(ts: impl Into<EcoVec<EuType<'eu>>>) -> Self {
        Self::Vec(ts.into())
    }

    #[inline]
    pub fn expr(ts: impl Into<EcoVec<EuType<'eu>>>) -> Self {
        Self::Expr(ts.into())
    }

    #[inline]
    pub fn seq(it: impl Iterator<Item = EuType<'eu>> + Send + Sync + 'eu) -> EuSeq<'eu> {
        Arc::new(Mutex::new(Box::new(it.map(Ok))))
    }

    pub fn to_vec(self) -> anyhow::Result<EcoVec<Self>> {
        match self {
            Self::Vec(ts) | EuType::Expr(ts) => Ok(ts),
            Self::Str(s) => s.chars().map(|t| Ok(EuType::Char(t))).collect(),
            Self::Opt(o) => o.into_iter().map(|t| Ok(*t)).collect(),
            Self::Res(r) => r.into_iter().map(|t| Ok(*t)).collect(),
            Self::Seq(it) => (&mut *it.lock().unwrap()).collect(),
            _ => Ok(eco_vec![self]),
        }
    }

    pub fn to_expr(self) -> anyhow::Result<EcoVec<Self>> {
        match self {
            Self::Expr(ts) => Ok(ts),
            Self::Str(s) => euphrates.parse(&s).map_err(|e| anyhow!(e.into_inner())),
            _ => Ok(eco_vec![self]),
        }
    }

    pub fn to_seq(self) -> EuSeq<'eu> {
        match self {
            Self::Seq(it) => it,
            t => Arc::new(Mutex::new(t.to_iter())),
        }
    }

    pub fn to_iter(self) -> EuResIter<'eu> {
        match self {
            Self::Str(s) => Box::new(
                s.chars()
                    .collect_vec()
                    .into_iter()
                    .map(|t| Ok(Self::Char(t))),
            ),
            Self::Opt(o) => Box::new(o.into_iter().map(|t| Ok(*t))),
            Self::Res(r) => Box::new(r.into_iter().map(|t| Ok(*t))),
            Self::Vec(ts) => Box::new(ts.into_iter().map(Ok)),
            Self::Seq(it) => Self::take_iter(&mut it.lock().unwrap()),
            t => Box::new(iter::once(Ok(t))),
        }
    }

    #[inline]
    pub fn take_iter(it: &mut EuResIter<'eu>) -> EuResIter<'eu> {
        mem::replace(it, Box::new(iter::empty()))
    }

    #[inline]
    pub fn is_vecz(&self) -> bool {
        self.is_opt() || self.is_res() || self.is_vec() || self.is_seq()
    }

    pub fn map(
        self,
        mut f: impl FnMut(Self) -> anyhow::Result<Self> + Send + Sync + 'eu,
    ) -> anyhow::Result<Self> {
        match self {
            Self::Opt(o) => o.map(|t| f(*t)).transpose().map(Self::opt),
            Self::Res(r) => swap_errors(r.map(|t| f(*t).map(Box::new))).map(Self::Res),
            Self::Vec(ts) => ts.into_iter().map(f).try_collect().map(Self::Vec),
            Self::Seq(it) => {
                {
                    let mut guard = it.lock().unwrap();
                    *guard = Box::new(Self::take_iter(&mut guard).map(move |t| f(t?)));
                }
                Ok(Self::Seq(it))
            }
            _ => f(self),
        }
    }

    pub fn flat_map(
        self,
        mut f: impl FnMut(Self) -> anyhow::Result<Self> + Send + Sync + 'eu,
    ) -> anyhow::Result<Self> {
        match self {
            Self::Opt(o) => o
                .and_then(|t| match f(*t) {
                    Ok(Self::Opt(o)) => o.map(Ok),
                    t => Some(t.map(Box::new)),
                })
                .transpose()
                .map(Self::Opt),
            Self::Res(r) => swap_errors(r.and_then(|t| match f(*t) {
                Ok(Self::Res(r)) => r.map(Ok),
                t => Ok(t.map(Box::new)),
            }))
            .map(Self::Res),
            Self::Vec(ts) => ts
                .into_iter()
                .flat_map(|t| match f(t) {
                    Ok(t) => t.to_iter(),
                    e @ Err(_) => Box::new(iter::once(e)),
                })
                .try_collect()
                .map(Self::Vec),
            Self::Seq(it) => {
                {
                    let mut guard = it.lock().unwrap();
                    *guard = Box::new(Self::take_iter(&mut guard).flat_map(move |r| {
                        match if let Ok(t) = r { f(t) } else { r } {
                            Ok(t) => t.to_iter(),
                            e @ Err(_) => Box::new(iter::once(e)),
                        }
                    }));
                }
                Ok(Self::Seq(it))
            }
            _ => f(self),
        }
    }

    pub fn zip(
        self,
        t: Self,
        mut f: impl FnMut(Self, Self) -> anyhow::Result<Self> + Send + Sync + 'eu,
    ) -> anyhow::Result<Self> {
        match (self, t) {
            (Self::Opt(a), Self::Opt(b)) => {
                a.zip(b).map(|(a, b)| f(*a, *b)).transpose().map(Self::opt)
            }
            (Self::Res(Ok(a)), Self::Res(Ok(b))) => f(*a, *b).map(|t| Self::res(Ok(t))),
            (Self::Res(a), Self::Res(b)) => Ok(Self::Res(a.and(b))),
            (Self::Vec(a), Self::Vec(b)) => a
                .into_iter()
                .zip(b)
                .map(|(a, b)| f(a, b))
                .try_collect()
                .map(Self::Vec),
            (Self::Seq(a), Self::Seq(b)) => {
                if Arc::ptr_eq(&a, &b) {
                    Self::Seq(a).map(move |a| {
                        let b = a.clone();
                        f(a, b)
                    })
                } else {
                    {
                        let mut guard = a.lock().unwrap();
                        let old = Self::take_iter(&mut b.lock().unwrap());
                        *guard = Box::new(
                            Self::take_iter(&mut guard)
                                .zip(old)
                                .map(move |(a, b)| a.and_then(|a| b.and_then(|b| f(a, b)))),
                        )
                    }
                    Ok(Self::Seq(a))
                }
            }
            (Self::Opt(a), b) => a.map(|t| f(*t, b)).transpose().map(Self::opt),
            (Self::Res(a), b) => swap_errors(a.map(|t| f(*t, b).map(Box::new))).map(Self::Res),
            (a, b) if a.is_vecz() => a.map(move |t| f(t, b.clone())),
            (a, Self::Opt(b)) => b.map(|t| f(a, *t)).transpose().map(Self::opt),
            (a, Self::Res(b)) => swap_errors(b.map(|t| f(a, *t).map(Box::new))).map(Self::Res),
            (a, b) if b.is_vecz() => b.map(move |t| f(a.clone(), t)),
            (a, b) => f(a, b),
        }
    }

    pub fn fold(
        self,
        init: Self,
        mut f: impl FnMut(Self, Self) -> anyhow::Result<Self> + Send + Sync + 'eu,
    ) -> anyhow::Result<Self> {
        match self {
            Self::Opt(Some(t)) | Self::Res(Ok(t)) => f(init, *t),
            Self::Opt(None) | Self::Res(Err(_)) => Ok(init),
            Self::Vec(ts) => ts.into_iter().try_fold(init, f),
            Self::Seq(it) => {
                Self::take_iter(&mut it.lock().unwrap()).try_fold(init, |acc, x| f(acc, x?))
            }
            _ => f(self, init),
        }
    }
}

impl PartialEq for EuType<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::I32(l0), Self::I32(r0)) => l0 == r0,
            (Self::I64(l0), Self::I64(r0)) => l0 == r0,
            (Self::F32(l0), Self::F32(r0)) => l0 == r0,
            (Self::F64(l0), Self::F64(r0)) => l0 == r0,
            (Self::Char(l0), Self::Char(r0)) => l0 == r0,
            (Self::Str(l0), Self::Str(r0)) | (Self::Word(l0), Self::Word(r0)) => l0 == r0,
            (Self::Opt(l0), Self::Opt(r0)) => l0 == r0,
            (Self::Res(l0), Self::Res(r0)) => l0 == r0,
            (Self::Vec(l0), Self::Vec(r0)) | (Self::Expr(l0), Self::Expr(r0)) => l0 == r0,
            (Self::Seq(l0), Self::Seq(r0)) => Arc::ptr_eq(l0, r0),
            _ => false,
        }
    }
}

#[crabtime::function]
fn gen_fn_to_num() {
    let types = ["I32", "I64", "F32", "F64"];
    for t in types {
        let tl = t.to_lowercase();
        let tlq = format!(r#""{tl}""#);
        let arms = types
            .map(|t| {
                crabtime::quote! {
                    EuType::{{t}}(n) => n.to_{{tl}}(),
                }
            })
            .join("");

        crabtime::output! {
            impl EuType<'_> {
                #[inline]
                pub fn to_res_{{tl}}(self) -> anyhow::Result<{{tl}}> {
                    self.to_{{tl}}().ok_or_else(|| anyhow!(concat!({{tlq}}, " conversion failed")))
                }

                pub fn to_{{tl}}(self) -> Option<{{tl}}> {
                    match self {
                        {{arms}}
                        EuType::Bool(b) => Some(b.into()),
                        EuType::Char(c) => (c as u32).to_{{tl}}(),
                        EuType::Str(s) => s.parse().ok(),
                        _ => None,
                    }
                }
            }
        }
    }
}

gen_fn_to_num!();

#[crabtime::function]
fn gen_fn_to_size() {
    let types = ["I32", "I64", "F32", "F64"];
    for n in ["isize", "usize"] {
        let nq = format!(r#""{n}""#);
        let arms = types
            .map(|t| {
                crabtime::quote! {
                    EuType::{{t}}(n) => n.to_{{n}}(),
                }
            })
            .join("");

        crabtime::output! {
            impl EuType<'_> {
                #[inline]
                pub fn to_res_{{n}}(self) -> anyhow::Result<{{n}}> {
                    self.to_{{n}}().ok_or_else(|| anyhow!(concat!({{nq}}, " conversion failed")))
                }

                pub fn to_{{n}}(self) -> Option<{{n}}> {
                    match self {
                        {{arms}}
                        EuType::Bool(b) => Some(b.into()),
                        EuType::Char(c) => (c as u32).to_{{n}}(),
                        EuType::Str(s) => s.parse().ok(),
                        _ => None
                    }
                }
            }
        }
    }
}

gen_fn_to_size!();

#[crabtime::function]
fn gen_fn_to_bool() {
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
                    EuType::Char(c) => c != '\0',
                    EuType::Str(s) => !s.is_empty(),
                    EuType::Word(_) => true,
                    EuType::Opt(o) => o.is_some(),
                    EuType::Res(r) => r.is_ok(),
                    EuType::Vec(ts) | EuType::Expr(ts) => !ts.is_empty(),
                    EuType::Seq(it) => Iterator::peekable(&mut *it.lock().unwrap()).peek().is_some(),
                }
            }
        }
    }
}

gen_fn_to_bool!();

#[crabtime::function]
fn gen_fn_neg() {
    let types = ["I32", "I64", "F32", "F64"];
    let arms = types
        .map(|t| {
            let n = t.to_lowercase();
            crabtime::quote! {
                EuType::{{t}}(n) => EuType::{{t}}(-n),
            }
        })
        .join("");

    crabtime::output! {
        impl Neg for EuType<'_> {
            type Output = Self;

            fn neg(self) -> Self {
                match self {
                    {{arms}}
                    EuType::Bool(b) => EuType::I32(b.into()).neg(),
                    EuType::Char(c) => EuType::I32(c as i32).neg(),
                    EuType::Str(s) => EuType::opt(s.parse().ok().map(|t: f64| EuType::F64(t).neg())),
                    _ if self.is_vecz() => self.map(|t| Ok(Self::neg(t))).unwrap(),
                    _ => EuType::Opt(None),
                }
            }
        }
    }
}

gen_fn_neg!();

#[crabtime::function]
fn gen_fn_math_binops() {
    use itertools::Itertools;

    let types0 = ["I32", "I64", "F32", "F64"];
    let types1 = [("Bool", "u8"), ("Char", "i32")];
    let types2 = ["Str"];

    for name in ["Add", "Sub", "Mul", "Div"] {
        let f = name.to_lowercase();

        let arms_num = itertools::repeat_n(types0, 2)
            .multi_cartesian_product()
            .map(|ts| {
                let t0 = ts[0];
                let t1 = ts[1];
                let c = types0[std::cmp::max(
                    types0.iter().position(|&t| t == t0).unwrap(),
                    types0.iter().position(|&t| t == t1).unwrap(),
                )];
                let n = c.to_lowercase();
                let f_chk = format!("checked_{f}");
                if t0.chars().next() == Some('I') && t1.chars().next() == Some('I') {
                    crabtime::quote! {
                        (EuType::{{t0}}(a), EuType::{{t1}}(b)) => {
                            EuType::opt((a as {{n}}).{{f_chk}}(b as {{n}}).map(EuType::{{c}}))
                        }
                    }
                } else {
                    crabtime::quote! {
                        (EuType::{{t0}}(a), EuType::{{t1}}(b)) => EuType::{{c}}((a as {{n}}).{{f}}(b as {{n}})),
                    }
                }
            })
            .join("");

        let arms_as = types0
            .iter()
            .cartesian_product(types1)
            .map(|(t0, (t1, n1))| {
                let n0 = t0.to_lowercase();
                crabtime::quote! {
                    (EuType::{{t0}}(a), EuType::{{t1}}(b)) => EuType::{{t0}}(a.{{f}}(b as {{n1}} as {{n0}})),
                    (EuType::{{t1}}(a), EuType::{{t0}}(b)) => EuType::{{t0}}((a as {{n1}} as {{n0}}).{{f}}(b)),
                }
            })
            .join("");

        let arms_parse = types0
            .iter()
            .cartesian_product(types2)
            .map(|(t0, t1)| {
                let n0 = t0.to_lowercase();
                crabtime::quote! {
                    (EuType::{{t0}}(a), EuType::{{t1}}(b)) => EuType::opt(
                        b.parse().ok().map(|t: {{n0}}| EuType::{{t0}}(a.{{f}}(t)))
                    ),
                    (EuType::{{t1}}(a), EuType::{{t0}}(b)) => EuType::opt(
                        a.parse().ok().map(|t: {{n0}}| EuType::{{t0}}(t.{{f}}(b)))
                    ),
                }
            })
            .join("");

        crabtime::output! {
            impl {{name}} for EuType<'_> {
                type Output = Self;

                fn {{f}}(self, rhs: Self) -> Self {
                    match (self, rhs) {
                        {{arms_num}}
                        (EuType::Bool(a), EuType::Bool(b)) => EuType::I32((a as i32).{{f}}(b as i32)),
                        (EuType::Char(a), EuType::Char(b)) => EuType::I32((a as i32).{{f}}(b as i32)),
                        {{arms_as}}
                        (EuType::Str(a), EuType::Str(b)) => EuType::opt((|| {
                            let a: f64 = a.parse().ok()?;
                            let b: f64 = b.parse().ok()?;
                            Some(EuType::F64(a.{{f}}(b)))
                        })()),
                        {{arms_parse}}
                        (a, b) if a.is_vecz() || b.is_vecz() => a.zip(b, |a, b| Ok(Self::{{f}}(a, b))).unwrap(),
                        _ => EuType::Opt(None),
                    }
                }
            }
        }
    }
}

gen_fn_math_binops!();
