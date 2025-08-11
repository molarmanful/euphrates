use std::{
    cmp::Ordering,
    iter,
    ops::{
        Add,
        Div,
        Mul,
        Neg,
        Sub,
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
use num_traits::{
    AsPrimitive,
    Signed,
    ToPrimitive,
};
use ordered_float::OrderedFloat;
use winnow::Parser;

use super::{
    EuRes,
    EuSeq,
};
use crate::{
    env::{
        EuEnv,
        EuScope,
    },
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
        self.is_i_32() || self.is_i_64() || self.is_f_32() || self.is_f_64()
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

    pub fn map<F>(self, mut f: F) -> EuRes<Self>
    where
        F: FnMut(Self) -> EuRes<Self> + Clone + 'eu,
    {
        match self {
            Self::Vec(ts) => ts.into_iter().map(f).try_collect().map(Self::Vec),
            Self::Seq(it) => Ok(Self::seq(it.map(move |t| f(t?)))),
            _ => self.map_once(f),
        }
    }

    pub fn map_once<F>(self, f: F) -> EuRes<Self>
    where
        F: FnOnce(Self) -> EuRes<Self> + 'eu,
    {
        match self {
            Self::Opt(o) => o.map(|t| f(*t)).transpose().map(Self::opt),
            Self::Res(r) => swap_errors(r.map(|t| f(*t).map(Box::new))).map(Self::Res),
            _ => f(self),
        }
    }

    pub fn map_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        let f = f.to_expr()?;
        if self.is_many() {
            self.map(move |t| EuEnv::apply_n_1(f.clone(), &[t], scope.clone()))
        } else {
            self.map_once(|t| EuEnv::apply_n_1(f, &[t], scope))
        }
    }

    pub fn flat_map<F>(self, mut f: F) -> EuRes<Self>
    where
        F: FnMut(Self) -> EuRes<Self> + Clone + 'eu,
    {
        match self {
            Self::Vec(ts) => ts
                .into_iter()
                .flat_map(|t| match f(t) {
                    Ok(t) => t.to_seq(),
                    e @ Err(_) => Box::new(iter::once(e)),
                })
                .try_collect()
                .map(Self::Vec),
            Self::Seq(it) => Ok(Self::seq(it.flat_map(move |r| {
                match if let Ok(t) = r { f(t) } else { r } {
                    Ok(t) => t.to_seq(),
                    e => Box::new(iter::once(e)),
                }
            }))),
            _ => self.flat_map_once(f),
        }
    }

    pub fn flat_map_once<F>(self, f: F) -> EuRes<Self>
    where
        F: FnOnce(Self) -> EuRes<Self> + 'eu,
    {
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
            _ => f(self),
        }
    }

    pub fn flat_map_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        let f = f.to_expr()?;
        if self.is_many() {
            self.flat_map(move |t| EuEnv::apply_n_1(f.clone(), &[t], scope.clone()))
        } else {
            self.flat_map_once(|t| EuEnv::apply_n_1(f, &[t], scope))
        }
    }

    #[inline]
    pub fn flatten(self) -> EuRes<Self> {
        self.flat_map(Ok)
    }

    #[inline]
    pub fn flatten_rec(self) -> EuRes<Self> {
        if self.is_vecz() {
            self.flat_map(|t| t.flatten_rec())
        } else {
            Ok(self)
        }
    }

    pub fn zip<F>(self, t: Self, mut f: F) -> EuRes<Self>
    where
        F: FnMut(Self, Self) -> EuRes<Self> + Clone + 'eu,
    {
        match (self, t) {
            (Self::Vec(a), Self::Vec(b)) => a
                .into_iter()
                .zip(b)
                .map(|(a, b)| f(a, b))
                .try_collect()
                .map(Self::Vec),
            (Self::Seq(a), Self::Seq(b)) => {
                Ok(Self::seq(a.zip(b).map(move |(a, b)| {
                    a.and_then(|a| b.and_then(|b| f(a, b)))
                })))
            }
            (a, b) if a.is_vecz() => a.map(move |t| f(t, b.clone())),
            (a, b) if b.is_vecz() => b.map(move |t| f(a.clone(), t)),
            (a, b) => a.zip_once(b, f),
        }
    }

    pub fn zip_once<F>(self, t: Self, f: F) -> EuRes<Self>
    where
        F: FnOnce(Self, Self) -> EuRes<Self> + 'eu,
    {
        match (self, t) {
            (Self::Opt(a), Self::Opt(b)) => {
                a.zip(b).map(|(a, b)| f(*a, *b)).transpose().map(Self::opt)
            }
            (Self::Res(Ok(a)), Self::Res(Ok(b))) => f(*a, *b).map(|t| Self::res(Ok(t))),
            (Self::Res(a), Self::Res(b)) => Ok(Self::Res(a.and(b))),
            (a, b) if a.is_once() => a.map_once(|t| f(t, b)),
            (a, b) if b.is_once() => b.map_once(|t| f(a, t)),
            (a, b) => f(a, b),
        }
    }

    pub fn zip_env(self, t: Self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        let f = f.to_expr()?;
        if self.is_many() {
            self.zip(t, move |a, b| {
                EuEnv::apply_n_1(f.clone(), &[a, b], scope.clone())
            })
        } else {
            self.zip_once(t, |a, b| EuEnv::apply_n_1(f, &[a, b], scope))
        }
    }

    pub fn fold<F>(self, init: Self, mut f: F) -> EuRes<Self>
    where
        F: FnMut(Self, Self) -> EuRes<Self> + Clone + 'eu,
    {
        match self {
            Self::Vec(ts) => ts.into_iter().try_fold(init, f),
            Self::Seq(mut it) => it.try_fold(init, |acc, t| f(acc, t?)),
            _ => self.fold_once(init, f),
        }
    }

    pub fn fold_once<F>(self, init: Self, f: F) -> EuRes<Self>
    where
        F: FnOnce(Self, Self) -> EuRes<Self> + 'eu,
    {
        match self {
            Self::Opt(Some(t)) | Self::Res(Ok(t)) => f(init, *t),
            Self::Opt(None) | Self::Res(Err(_)) => Ok(init),
            _ => f(self, init),
        }
    }

    pub fn fold_env(self, init: Self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        let f = f.to_expr()?;
        if self.is_many() {
            self.fold(init, move |acc, t| {
                EuEnv::apply_n_1(f.clone(), &[acc, t], scope.clone())
            })
        } else {
            self.fold_once(init, |acc, t| EuEnv::apply_n_1(f, &[acc, t], scope))
        }
    }

    pub fn sorted(mut self) -> EuRes<Self> {
        match self {
            Self::Vec(ref mut ts) => {
                ts.make_mut().sort();
                Ok(self)
            }
            _ => Self::Vec(self.to_vec()?).sorted(),
        }
    }
}

#[crabtime::function]
fn gen_num_tower() {
    use itertools::Itertools;

    let types0 = ["I32", "I64", "F32", "F64"];
    let types1 = [("Bool", "I32"), ("Char", "I32")];

    let arms_same = types0
        .map(|t| {
            crabtime::quote! {
                t @ (Self::{{t}}(_), Self::{{t}}(_)) => Some(t),
            }
        })
        .join("");

    let arms_num = types0.iter().cloned().permutations(2)
        .map(|ts| {
            let t0 = ts[0];
            let t1 = ts[1];
            let c = types0.iter().rev().find(|&&t| t == t0 || t == t1).unwrap();
            let n = c.to_lowercase();
            if t0.chars().next() == Some('I') && t1.chars().next() == Some('I') {
                crabtime::quote! {
                    (Self::{{t0}}(a), Self::{{t1}}(b)) => Some((Self::{{c}}(a as {{n}}), Self::{{c}}(b as {{n}}))),
                }
            } else {
                crabtime::quote! {
                    (Self::{{t0}}(a), Self::{{t1}}(b)) => {
                        let a: OrderedFloat<{{n}}> = a.as_();
                        let b: OrderedFloat<{{n}}> = b.as_();
                        Some((Self::{{c}}(a), Self::{{c}}(b)))
                    }
                }
            }
        })
        .join("");

    let arms_like = types1
        .map(|(t0, t1)| {
            let n = t1.to_lowercase();
            crabtime::quote! {
                (Self::{{t0}}(t), b) => Self::{{t1}}(t as {{n}}).num_tower(b),
                (a, Self::{{t0}}(t)) => a.num_tower(Self::{{t1}}(t as {{n}})),
            }
        })
        .join("");

    let arms_parse = types0.map(|t| (t, t)).into_iter()
        .chain(types1)
        .map(|(t0, t1)| {
            crabtime::quote! {
                (Self::Str(s), b @ Self::{{t0}}(_)) => Self::{{t1}}(s.parse().ok()?).parse_num_tower(b),
                (a @ Self::{{t0}}(_), Self::Str(s)) => a.parse_num_tower(Self::{{t1}}(s.parse().ok()?)),
            }
        })
        .join("");

    crabtime::output! {
        impl EuType<'_> {
            pub fn num_tower(self, other: Self) -> Option<(Self, Self)> {
                match (self, other) {
                    {{arms_same}}
                    {{arms_num}}
                    {{arms_like}}
                    _ => None
                }
            }

            pub fn parse_num_tower(self, other: Self) -> Option<(Self, Self)> {
                match (self, other) {
                    (Self::Str(a), Self::Str(b)) => {
                        a.parse().ok()
                            .zip(b.parse().ok())
                            .map(|(a, b)| (Self::F64(a), Self::F64(b)))
                    }
                    {{arms_parse}}
                    (a, b) => a.num_tower(b)
                }
            }
        }
    }
}

gen_num_tower!();

#[crabtime::function]
fn gen_eqv_ord() {
    use itertools::Itertools;

    let types = [
        "Bool", "Char", "I32", "I64", "F32", "F64", "Word", "Str", "Opt", "Res", "Expr", "Vec",
        "Seq",
    ];

    let arms = itertools::repeat_n(types, 2)
        .multi_cartesian_product()
        .map(|ts| {
            let t0 = ts[0];
            let t1 = ts[1];
            let c = format!(
                "Ordering::{:?}",
                types
                    .iter()
                    .position(|&t| t == t0)
                    .unwrap()
                    .cmp(&types.iter().position(|&t| t == t1).unwrap())
            );
            crabtime::quote! {
                (Self::{{t0}}(a), Self::{{t1}}(b)) => {{c}},
            }
        })
        .join("");

    crabtime::output! {
        impl EuType<'_> {
            pub fn eqv_ord(&self, other: &Self) -> Ordering {
                match (self, other) {
                    {{arms}}
                }
            }
        }
    }
}

gen_eqv_ord!();

#[crabtime::function]
fn gen_partial_eq() {
    let types = [
        "Bool", "I32", "I64", "F32", "F64", "Char", "Str", "Word", "Opt", "Res", "Vec", "Expr",
    ];
    let arms = types
        .map(|t| {
            crabtime::quote! {
                (Self::{{t}}(l0), Self::{{t}}(r0)) => l0 == r0,
            }
        })
        .join("");

    crabtime::output! {
        impl PartialEq for EuType<'_> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    {{arms}}
                    (Self::Seq(l0), Self::Seq(r0)) => l0.clone().eq(r0.clone()),
                    (a, b) if a.is_num() && b.is_num() => {
                        let (a, b) = a.clone().num_tower(b.clone()).unwrap();
                        a == b
                    }
                    _ => false,
                }
            }
        }
    }
}

gen_partial_eq!();

impl Eq for EuType<'_> {}

impl PartialOrd for EuType<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[crabtime::function]
fn gen_ord() {
    use itertools::Itertools;

    let types = [
        "Bool", "I32", "I64", "F32", "F64", "Char", "Str", "Word", "Opt", "Res", "Vec", "Expr",
    ];
    let arms = types
        .map(|t| {
            crabtime::quote! {
                (Self::{{t}}(l0), Self::{{t}}(r0)) => l0.cmp(r0),
            }
        })
        .join("");

    let nums = ["I32", "I64", "F32", "F64"];
    let arms_num = nums
        .iter()
        .permutations(2)
        .map(|ts| {
            let t0 = *ts[0];
            let t1 = *ts[1];
            let m = *nums.iter().find(|&&t| t == t0 || t == t1).unwrap();
            let n = if t0 == m { t1 } else { t0 };
            if t0.chars().next().unwrap() == t1.chars().next().unwrap() {
                crabtime::quote! {
                    (Self::{{t0}}(l0), Self::{{t1}}(r0)) => Self::{{n}}((*l0).as_()).cmp(&Self::{{n}}((*r0).as_())),
                }
            } else if t0 == n {
                let m = m.to_lowercase();
                let n = n.to_lowercase();
                crabtime::quote! {
                    (Self::{{t0}}(l0), Self::{{t1}}(r0)) => {
                        if l0.to_{{m}}().is_none() {
                            l0.cmp(&0.0.into())
                        } else {
                            let r0: OrderedFloat<{{n}}> = r0.to_{{n}}().unwrap().into();
                            l0.cmp(&r0)
                        }
                    }
                }
            } else {
                crabtime::quote! {
                    (l0 @ Self::{{t0}}(_), r0 @ Self::{{t1}}(_)) => r0.cmp(l0).reverse(),
                }
            }
        })
        .join("\n");

    crabtime::output! {
        impl Ord for EuType<'_> {
            fn cmp(&self, other: &Self) -> Ordering {
                match (self, other) {
                    {{arms}}
                    (Self::Seq(l0), Self::Seq(r0)) => l0.clone().cmp(r0.clone()),
                    (Self::Bool(l0), _) => l0.cmp(&!l0),
                    (Self::Word(_), _) => Ordering::Greater,
                    (l0, r0 @ (Self::Bool(_) | Self::Word(_))) => r0.cmp(l0).reverse(),
                    {{arms_num}}
                    (a, b) if a.is_num_like() && b.is_num_like() => {
                        let (a1, b1) = a.clone().num_tower(b.clone()).unwrap();
                        a1.cmp(&b1).then_with(|| a.eqv_ord(b))
                    }
                    (a, b) if a.is_vecz() || a.is_str() || a.is_expr() || b.is_vecz() || b.is_str() || b.is_expr() => {
                        a.clone().to_seq().cmp(b.clone().to_seq()).then_with(|| a.eqv_ord(b))
                    }
                    (a, b) => panic!("order not defined between {a:?} and {b:?}"),
                }
            }
        }
    }
}

gen_ord!();

#[crabtime::function]
fn gen_type_to_num() {
    let types = ["I32", "I64", "F32", "F64"];
    for t in types {
        let tl = t.to_lowercase();
        let tlq = format!(r#""{tl}""#);
        let arms = types
            .map(|t| {
                crabtime::quote! {
                    Self::{{t}}(n) => n.to_{{tl}}(),
                }
            })
            .join("");

        crabtime::output! {
            impl EuType<'_> {
                #[inline]
                pub fn to_res_{{tl}}(self) -> EuRes<{{tl}}> {
                    self.to_{{tl}}().ok_or_else(|| anyhow!(concat!({{tlq}}, " conversion failed")).into())
                }

                pub fn to_{{tl}}(self) -> Option<{{tl}}> {
                    match self {
                        {{arms}}
                        Self::Bool(b) => Some(b.into()),
                        Self::Char(c) => (c as u32).to_{{tl}}(),
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
    let types = ["I32", "I64", "F32", "F64"];
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
                pub fn to_res_{{n}}(self) -> EuRes<{{n}}> {
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

#[crabtime::function]
fn gen_impl_neg() {
    let types = ["I32", "I64", "F32", "F64"];
    let arms = types
        .map(|t| {
            let n = t.to_lowercase();
            crabtime::quote! {
                Self::{{t}}(n) => Self::{{t}}(-n),
            }
        })
        .join("");

    crabtime::output! {
        impl Neg for EuType<'_> {
            type Output = Self;

            fn neg(self) -> Self {
                match self {
                    {{arms}}
                    Self::Bool(b) => -Self::I32(b.into()),
                    Self::Char(c) => -Self::I32(c as i32),
                    Self::Str(s) => Self::opt(s.parse().ok().map(|t| -Self::F64(t))),
                    _ if self.is_vecz() => self.map(|t| Ok(-t)).unwrap(),
                    _ => Self::Opt(None),
                }
            }
        }
    }
}

gen_impl_neg!();

#[crabtime::function]
fn gen_math_binops() {
    use itertools::Itertools;

    let types = ["I32", "I64", "F32", "F64"];
    for name in ["Add", "Sub", "Mul", "Div"] {
        let f = name.to_lowercase();
        let fq = format!(r#""{f}""#);

        let arms = types
            .map(|t| {
                if t.chars().next().unwrap() == 'I' {
                    crabtime::quote! {
                        (Self::{{t}}(a), Self::{{t}}(b)) => {
                            a.checked_{{f}}(b)
                                .map(Self::{{t}})
                                .ok_or_else(|| anyhow!("{} on `{a:?}` and `{b:?}` is undefined", {{fq}}).into())
                        }
                    }
                } else {
                    crabtime::quote! {
                        (Self::{{t}}(a), Self::{{t}}(b)) => Ok(Self::{{t}}(a.{{f}}(b))),
                    }
                }
            })
            .join("");

        crabtime::output! {
            impl {{name}} for EuType<'_> {
                type Output = EuRes<Self>;

                fn {{f}}(self, rhs: Self) -> Self::Output {
                    match (self, rhs) {
                        {{arms}}
                        (a, b) if a.is_num_like() && b.is_num_like() => {
                            let (a, b) = a.num_tower(b).unwrap();
                            a.{{f}}(b)
                        }
                        (a, b) if a.is_num_parse() && b.is_num_parse() => {
                            a.parse_num_tower(b)
                                .ok_or_else(|| anyhow!(concat!("failed to parse before ", {{fq}})).into())
                                .and_then(|(a, b)| a.{{f}}(b))
                        }
                        (a, b) if a.is_vecz() || b.is_vecz() => a.zip(b, |a, b| a.{{f}}(b)),
                        (a, b) => Err(anyhow!("cannot {} `{a:?}` and `{b:?}`", {{fq}}).into()),
                    }
                }
            }
        }
    }
}

gen_math_binops!();
