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
use hipstr::HipStr;
use itertools::Itertools;
use num_traits::ToPrimitive;
use winnow::Parser;

use crate::parser::euphrates;

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
    Str(Box<HipStr<'eu>>),
    #[debug("{_0}")]
    Word(Box<HipStr<'eu>>),

    #[debug("{}", if let Some(t) = _0 { format!("Some:{t:?}") } else { "None".into() })]
    #[display("{}", if let Some(t) = _0 { t.to_string() } else { "".to_string() })]
    Opt(Option<Box<EuType<'eu>>>),
    #[debug("{}", match _0 { Ok(t) => format!("Ok:{t:?}"), Err(e) => format!("Err:{e:?}") })]
    #[display("{}", match _0 { Ok(t) => t.to_string(), Err(e) => e.to_string() })]
    Res(Result<Box<EuType<'eu>>, Box<EuType<'eu>>>),

    #[debug("Vec:({})", _0.iter().map(|t| format!("{t:?}")).join(" "))]
    #[display("{}", _0.iter().join(""))]
    Vec(Box<imbl::Vector<EuType<'eu>>>),
    #[debug("({})", _0.iter().map(|t| format!("{t:?}")).join(" "))]
    #[display("{}", _0.iter().join(" "))]
    Expr(Box<imbl::Vector<EuType<'eu>>>),
    #[debug("Seq:(...)")]
    #[display("{}", _0.lock().unwrap().join(""))]
    Seq(EuSeq<'eu>),
}

type EuSeq<'eu> = Arc<Mutex<EuIter<'eu>>>;

pub type EuIter<'eu> = Box<dyn Iterator<Item = EuType<'eu>> + Send + Sync + 'eu>;

impl<'eu> EuType<'eu> {
    pub fn str(s: impl Into<HipStr<'eu>>) -> Self {
        Self::Str(Box::new(s.into()))
    }

    pub fn word(s: impl Into<HipStr<'eu>>) -> Self {
        Self::Word(Box::new(s.into()))
    }

    pub fn opt(s: Option<EuType<'eu>>) -> Self {
        Self::Opt(s.map(Box::new))
    }

    pub fn vec(ts: impl Into<imbl::Vector<EuType<'eu>>>) -> Self {
        Self::Vec(Box::new(ts.into()))
    }

    pub fn expr(ts: impl Into<imbl::Vector<EuType<'eu>>>) -> Self {
        Self::Expr(Box::new(ts.into()))
    }

    pub fn seq(it: impl Iterator<Item = EuType<'eu>> + Send + Sync + 'eu) -> EuSeq<'eu> {
        Arc::new(Mutex::new(Box::new(it)))
    }

    pub fn to_vec(self) -> imbl::Vector<Self> {
        match self {
            EuType::Vec(ts) | EuType::Expr(ts) => *ts,
            EuType::Str(s) => s.chars().map(EuType::Char).collect(),
            EuType::Opt(o) => o.into_iter().map(|t| *t).collect(),
            EuType::Res(r) => r.into_iter().map(|t| *t).collect(),
            EuType::Seq(it) => (&mut *it.lock().unwrap()).collect(),
            _ => imbl::vector![self],
        }
    }

    pub fn to_expr(self) -> anyhow::Result<imbl::Vector<Self>> {
        match self {
            EuType::Str(s) => euphrates.parse(&s).map_err(|e| anyhow!(e.into_inner())),
            _ if self.is_vecz() => Ok(imbl::vector![self]),
            _ => Ok(self.to_vec()),
        }
    }

    pub fn to_seq(self) -> EuSeq<'eu> {
        match self {
            EuType::Str(s) => Self::seq(s.chars().collect_vec().into_iter().map(EuType::Char)),
            EuType::Opt(o) => Self::seq(o.into_iter().map(|t| *t)),
            EuType::Res(r) => Self::seq(r.into_iter().map(|t| *t)),
            EuType::Vec(ts) | EuType::Expr(ts) => Self::seq(ts.clone().into_iter()),
            EuType::Seq(it) => it,
            _ => Self::seq(iter::once(self)),
        }
    }

    pub fn take_iter(it: &mut EuIter<'eu>) -> EuIter<'eu> {
        mem::replace(it, Box::new(iter::empty()))
    }

    pub fn is_vecz(&self) -> bool {
        self.is_opt() || self.is_res() || self.is_vec() || self.is_seq()
    }

    pub fn map(self, mut f: impl FnMut(Self) -> Self + Send + Sync + 'eu) -> Self {
        match self {
            Self::Opt(o) => Self::Opt(o.map(|t| Box::new(f(*t)))),
            Self::Res(r) => Self::Res(r.map(|t| Box::new(f(*t)))),
            Self::Vec(ts) => Self::Vec(Box::new(ts.into_iter().map(f).collect())),
            Self::Seq(it) => {
                {
                    let mut guard = it.lock().unwrap();
                    *guard = Box::new(Self::take_iter(&mut guard).map(f));
                }
                Self::Seq(it)
            }
            _ => f(self),
        }
    }

    pub fn zip(self, t: Self, mut f: impl FnMut(Self, Self) -> Self + Send + Sync + 'eu) -> Self {
        match (self, t) {
            (Self::Opt(a), Self::Opt(b)) => Self::Opt(a.zip(b).map(|(a, b)| Box::new(f(*a, *b)))),
            (Self::Res(Ok(a)), Self::Res(Ok(b))) => Self::Res(Ok(Box::new(f(*a, *b)))),
            (Self::Res(a), Self::Res(b)) => Self::Res(a.and(b)),
            (Self::Vec(a), Self::Vec(b)) => Self::Vec(Box::new(
                a.into_iter().zip(*b).map(|(a, b)| f(a, b)).collect(),
            )),
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
                                .map(move |(a, b)| f(a, b)),
                        )
                    }
                    Self::Seq(a)
                }
            }
            (Self::Opt(a), b) => Self::Opt(a.map(|t| Box::new(f(*t, b)))),
            (Self::Res(a), b) => Self::Res(a.map(|t| Box::new(f(*t, b)))),
            (a, b) if a.is_vecz() => a.map(move |t| f(t, b.clone())),
            (a, Self::Opt(b)) => Self::Opt(b.map(|t| Box::new(f(a, *t)))),
            (a, Self::Res(b)) => Self::Res(b.map(|t| Box::new(f(a, *t)))),
            (a, b) if b.is_vecz() => b.map(move |t| f(a.clone(), t)),
            (a, b) => f(a, b),
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
            (Self::Seq(l0), Self::Seq(r0)) => {
                Arc::ptr_eq(l0, r0) || (&mut *l0.lock().unwrap()).eq(&mut *r0.lock().unwrap())
            }
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
                    EuType::Str(s) => EuType::Opt(s.parse().ok().map(|t: f64| Box::new(EuType::F64(t).neg()))),
                    _ if self.is_vecz() => self.map(Self::neg),
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
                            EuType::Opt((a as {{n}}).{{f_chk}}(b as {{n}}).map(|t| Box::new(EuType::{{c}}(t))))
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
                    (EuType::{{t0}}(a), EuType::{{t1}}(b)) => EuType::Opt(
                        b.parse().ok().map(|t: {{n0}}| Box::new(EuType::{{t0}}(a.{{f}}(t))))
                    ),
                    (EuType::{{t1}}(a), EuType::{{t0}}(b)) => EuType::Opt(
                        a.parse().ok().map(|t: {{n0}}| Box::new(EuType::{{t0}}(t.{{f}}(b))))
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
                        (EuType::Str(a), EuType::Str(b)) => EuType::Opt((|| {
                            let a: f64 = a.parse().ok()?;
                            let b: f64 = b.parse().ok()?;
                            Some(Box::new(EuType::F64(a.{{f}}(b))))
                        })()),
                        {{arms_parse}}
                        (a, b) if a.is_vecz() || b.is_vecz() => a.zip(b, Self::{{f}}),
                        _ => EuType::Opt(None),
                    }
                }
            }
        }
    }
}

gen_fn_math_binops!();
