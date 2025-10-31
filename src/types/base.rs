use std::{
    hash::Hash,
    iter,
    mem,
    rc::Rc,
};

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
use hipstr::LocalHipStr;
use itertools::Itertools;
use num_traits::{
    Signed,
    ToPrimitive,
    Zero,
};
use ordered_float::OrderedFloat;
use ordermap::{
    OrderMap,
    OrderSet,
};
use winnow::Parser;

use super::{
    EuRes,
    EuSeq,
    EuSyn,
};
use crate::parser::euphrates;

#[derive(Debug, Display, Hash, Clone, IsVariant)]
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
    Str(LocalHipStr<'eu>),
    #[debug("{_0}")]
    Word(LocalHipStr<'eu>),

    #[debug("{}", if let Some(t) = _0 { format!("Some:{t:?}") } else { "None".into() })]
    #[display("{}", if let Some(t) = _0 { t.to_string() } else { "".to_string() })]
    Opt(Option<Box<Self>>),
    #[debug("{}", match _0 { Ok(t) => format!("Ok:{t:?}"), Err(e) => format!("Err:{e:?}") })]
    #[display("{}", match _0 { Ok(t) => t.to_string(), Err(e) => e.to_string() })]
    Res(Result<Box<Self>, Box<Self>>),

    #[debug("[{}]", _0.iter().map(|t| format!("{t:?}")).join(" "))]
    #[display("{}", _0.iter().join(""))]
    Vec(EcoVec<Self>),
    #[debug("{{{}}}", _0.iter().map(|(k, v)| format!("{k:?} => {v:?}")).join(", "))]
    #[display("{}", _0.iter().map(|(k, v)| format!("{k:?}{v:?}")).join(" "))]
    Map(Rc<OrderMap<Self, Self>>),
    #[debug("Set:({})", _0.iter().map(|t| format!("{t:?}")).join(", "))]
    #[display("{}", _0.iter().join(""))]
    Set(Rc<OrderSet<Self>>),
    #[debug("({})", _0.iter().map(|t| format!("{t:?}")).join(" "))]
    #[display("{}", _0.iter().join(" "))]
    Expr(EcoVec<EuSyn<'eu>>),
    #[debug("Seq:(...)")]
    #[display("{}", match _0.clone().try_collect::<_, Vec<_>, _>() { Ok(ts) => ts.into_iter().join(""), Err(e) => "".into() })]
    Seq(EuSeq<'eu>),
}

impl Default for EuType<'_> {
    fn default() -> Self {
        Self::Opt(None)
    }
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
    pub fn str(s: impl Into<LocalHipStr<'eu>>) -> Self {
        Self::Str(s.into())
    }

    #[inline]
    pub fn word(s: impl Into<LocalHipStr<'eu>>) -> Self {
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
    pub fn map_(ts: impl Into<OrderMap<Self, Self>>) -> Self {
        Self::Map(Rc::new(ts.into()))
    }

    #[inline]
    pub fn set(ts: impl Into<OrderSet<Self>>) -> Self {
        Self::Set(Rc::new(ts.into()))
    }

    #[inline]
    pub fn expr(ts: impl Into<EcoVec<EuSyn<'eu>>>) -> Self {
        Self::Expr(ts.into())
    }

    #[inline]
    pub fn seq<I>(it: I) -> Self
    where
        I: Iterator<Item = EuRes<Self>> + Clone + 'eu,
    {
        Self::Seq(Box::new(it))
    }

    pub fn to_pair(self) -> EuRes<(Self, Self)> {
        let mut seq = self.to_seq();
        match (seq.next().transpose()?, seq.next().transpose()?) {
            (Some(k), Some(v)) => Ok((k, v)),
            _ => Err(anyhow!("failed to convert to pair").into()),
        }
    }

    pub fn to_opt(self) -> Option<Self> {
        match self {
            Self::Opt(o) => o.map(|t| *t),
            Self::Res(r) => r.ok().map(|t| *t),
            _ => Some(self),
        }
    }

    pub fn to_expr(self) -> EuRes<EcoVec<EuSyn<'eu>>> {
        match self {
            Self::Expr(ts) => Ok(ts),
            Self::Str(s) => euphrates
                .parse(&s)
                .map_err(|e| anyhow!(e.into_inner()).into()),
            _ => Ok(eco_vec![EuSyn::Raw(self)]),
        }
    }

    pub fn to_vec(self) -> EuRes<EcoVec<Self>> {
        match self {
            Self::Vec(ts) => Ok(ts),
            Self::Map(kvs) => Rc::unwrap_or_clone(kvs)
                .into_iter()
                .map(|(k, v)| Ok(Self::vec([k, v])))
                .collect(),
            Self::Set(ts) => Ok(Rc::unwrap_or_clone(ts).into_iter().collect()),
            Self::Seq(it) => it.collect(),
            Self::Opt(o) => o.into_iter().map(|t| Ok(*t)).collect(),
            Self::Res(r) => r.into_iter().map(|t| Ok(*t)).collect(),
            Self::Expr(ts) => Ok(ts.into_iter().map(EuSyn::into).collect()),
            Self::Str(s) => s.chars().map(|c| Ok(Self::Char(c))).collect(),
            _ => Ok(eco_vec![self]),
        }
    }

    pub fn to_seq(self) -> EuSeq<'eu> {
        match self {
            Self::Seq(it) => it,
            Self::Vec(ts) => Box::new(ts.as_slice().to_vec().into_iter().map(Ok)),
            Self::Map(kvs) => Box::new(
                Rc::unwrap_or_clone(kvs)
                    .into_iter()
                    .map(|(k, v)| Ok(Self::vec([k, v]))),
            ),
            Self::Set(ts) => Box::new(Rc::unwrap_or_clone(ts).into_iter().map(Ok)),
            Self::Opt(o) => Box::new(o.into_iter().map(|t| Ok(*t))),
            Self::Res(r) => Box::new(r.into_iter().map(|t| Ok(*t))),
            Self::Expr(ts) => Box::new(ts.as_slice().to_vec().into_iter().map(|t| Ok(t.into()))),
            Self::Str(s) => Box::new(
                s.chars()
                    .collect_vec()
                    .into_iter()
                    .map(|c| Ok(Self::Char(c))),
            ),
            _ => Box::new(iter::once(Ok(self))),
        }
    }

    pub fn to_map(self) -> EuRes<Rc<OrderMap<Self, Self>>> {
        match self {
            Self::Map(kvs) => Ok(kvs),
            Self::Vec(ts) => Ok(Rc::new(
                ts.into_iter()
                    .enumerate()
                    .map(|(i, t)| (Self::I64(i as i64), t))
                    .collect(),
            )),
            Self::Set(ts) => Ok(Rc::new(
                Rc::unwrap_or_clone(ts)
                    .into_iter()
                    .map(|k| (k, Self::Bool(true)))
                    .collect(),
            )),
            Self::Seq(it) => it
                .enumerate()
                .map(|(i, r)| r.map(|t| (Self::I64(i as i64), t)))
                .try_collect()
                .map(Rc::new),
            Self::Opt(o) => Ok(Rc::new(
                o.into_iter()
                    .enumerate()
                    .map(|(i, t)| (Self::I64(i as i64), *t))
                    .collect(),
            )),
            Self::Res(r) => Ok(Rc::new(
                r.into_iter()
                    .enumerate()
                    .map(|(i, t)| (Self::I64(i as i64), *t))
                    .collect(),
            )),
            Self::Expr(ts) => Ok(Rc::new(
                ts.into_iter()
                    .enumerate()
                    .map(|(i, t)| (Self::I64(i as i64), t.into()))
                    .collect(),
            )),
            Self::Str(s) => Ok(Rc::new(
                s.chars()
                    .enumerate()
                    .map(|(i, c)| (Self::I64(i as i64), Self::Char(c)))
                    .collect(),
            )),
            _ => Ok(Rc::new([(Self::I64(0), self)].into())),
        }
    }

    pub fn to_set(self) -> EuRes<Rc<OrderSet<Self>>> {
        match self {
            Self::Set(ts) => Ok(ts),
            Self::Vec(ts) => Ok(Rc::new(ts.into_iter().collect())),
            Self::Map(kvs) => Ok(Rc::new(Rc::unwrap_or_clone(kvs).into_keys().collect())),
            Self::Seq(it) => it.try_collect().map(Rc::new),
            Self::Opt(o) => Ok(Rc::new(o.into_iter().map(|t| *t).collect())),
            Self::Res(r) => Ok(Rc::new(r.into_iter().map(|t| *t).collect())),
            Self::Expr(ts) => Ok(Rc::new(ts.into_iter().map(EuSyn::into).collect())),
            Self::Str(s) => Ok(Rc::new(s.chars().map(Self::Char).collect())),
            _ => Ok(Rc::new([self].into())),
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
    pub fn is_int(&self) -> bool {
        self.is_i_32() || self.is_i_64() || self.is_i_big()
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
        self.is_vec() || self.is_seq() || self.is_map() || self.is_set()
    }

    pub fn push(mut self, other: Self) -> EuRes<Self> {
        match self {
            Self::Vec(ref mut ts) => {
                ts.push(other);
                Ok(self)
            }
            Self::Map(ref mut kvs) => {
                let (k, v) = other.to_pair()?;
                Rc::make_mut(kvs).insert(k, v);
                Ok(self)
            }
            Self::Set(ref mut ts) => {
                Rc::make_mut(ts).insert(other);
                Ok(self)
            }
            Self::Seq(it) => Ok(Self::seq(it.chain(iter::once(Ok(other))))),
            Self::Expr(ref mut ts) => {
                ts.push(EuSyn::Raw(other));
                Ok(self)
            }
            Self::Str(ref mut s) => {
                if let Self::Char(c) = other {
                    s.push(c);
                } else {
                    s.push_str(&other.to_string());
                };
                Ok(self)
            }
            _ => Ok(Self::vec([self, other])),
        }
    }

    #[inline]
    pub fn push_front(self, other: Self) -> EuRes<Self> {
        self.insert(0, other)
    }

    pub fn insert(mut self, index: isize, mut other: Self) -> EuRes<Self> {
        let a = index.unsigned_abs();
        let check = |len: usize| {
            let hi = len as isize;
            let low = -hi - 1;
            (low <= index && index <= hi)
                .ok_or_else(|| anyhow!("{index} out of bounds [{low}, {hi}]"))
                .map(|()| if index < 0 { len + 1 - a } else { a })
        };

        match self {
            Self::Vec(ref mut ts) => {
                ts.insert(check(ts.len())?, other);
                Ok(self)
            }
            Self::Map(ref mut kvs) => {
                let a = check(kvs.len())?;
                let (k, v) = other.to_pair()?;
                Rc::make_mut(kvs).insert_before(a, k, v);
                Ok(self)
            }
            Self::Set(ref mut ts) => {
                let a = check(ts.len())?;
                Rc::make_mut(ts).insert_before(a, other);
                Ok(self)
            }
            Self::Seq(it) => {
                if index < 0 {
                    Self::Vec(it.try_collect()?).insert(index, other)
                } else {
                    let mut i = 0;
                    let mut ins = false;
                    Ok(Self::seq(it.batching(move |it| {
                        if !ins && i == index {
                            ins = true;
                            Some(Ok(mem::take(&mut other)))
                        } else {
                            let t = it.next();
                            if t.is_some() {
                                i += 1;
                            }
                            t
                        }
                    })))
                }
            }
            Self::Expr(ref mut ts) => {
                ts.insert(check(ts.len())?, EuSyn::Raw(other));
                Ok(self)
            }
            Self::Str(s) => {
                let a = check(s.len())?;
                let mut res = s.slice(0..a);
                if let Self::Char(c) = other {
                    res.push(c);
                } else {
                    res.push_str(&other.to_string());
                };
                res.push_str(&s.slice(a..));
                Ok(Self::Str(res))
            }
            _ => Self::vec([self]).insert(index, other),
        }
    }

    pub fn append(self, other: Self) -> EuRes<Self> {
        match (self, other) {
            (Self::Map(a), Self::Map(b)) => {
                let mut a = Rc::unwrap_or_clone(a);
                a.extend(Rc::unwrap_or_clone(b));
                Ok(Self::map_(a))
            }
            (Self::Set(a), Self::Set(b)) => {
                let mut a = Rc::unwrap_or_clone(a);
                a.extend(Rc::unwrap_or_clone(b));
                Ok(Self::set(a))
            }
            (Self::Char(a), Self::Char(b)) => {
                let mut s = LocalHipStr::with_capacity(2);
                s.push(a);
                s.push(b);
                Ok(Self::Str(s))
            }
            (a, b) if a.is_seq() || b.is_seq() => Ok(Self::seq(a.to_seq().chain(b.to_seq()))),
            (a, b) if a.is_vec() || b.is_vec() => {
                let mut a = a.to_vec()?;
                a.extend(b.to_vec()?);
                Ok(Self::Vec(a))
            }
            (a, b) if a.is_expr() || b.is_expr() => {
                let mut a = a.to_expr()?;
                a.extend(b.to_expr()?);
                Ok(Self::Expr(a))
            }
            (a, b) if a.is_str() || b.is_str() => Ok(Self::str(format!("{a}{b}"))),
            (a, b) => Ok(Self::vec([a, b])),
        }
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
                    if t0 == "IBig" {
                        crabtime::quote! {
                            Self::{{t1}}(n) => Some(n.clone()),
                        }
                    } else if t0.chars().next() == Some('I') {
                        crabtime::quote! {
                            Self::{{t1}}(n) => Some(*n),
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
                            Self::{{t1}}(n) => Some((*n).into()),
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
                pub fn try_{{tl}}(&self) -> EuRes<{{tlp}}> {
                    self.to_{{tl}}().ok_or_else(move || {
                        anyhow!("failed to convert `{self:?}` to {}", {{tlq}}).into()
                    })
                }

                pub fn to_{{tl}}(&self) -> Option<{{tlp}}> {
                    match self {
                        {{arms}}
                        Self::Bool(b) => Some((*b).into()),
                        Self::Char(c) => Self::I32(*c as i32).to_{{tl}}(),
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
fn gen_type_to_num_other() {
    let types = ["I32", "I64", "IBig", "F32", "F64"];
    for n in ["isize", "usize", "u32", "u64"] {
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
                pub fn try_{{n}}(&self) -> EuRes<{{n}}> {
                    self.to_{{n}}().ok_or_else(move || {
                        anyhow!("failed to convert `{self:?}` to {}", {{nq}}).into()
                    })
                }

                pub fn to_{{n}}(&self) -> Option<{{n}}> {
                    match self {
                        {{arms}}
                        Self::Bool(b) => Some((*b).into()),
                        Self::Char(c) => (*c as u32).to_{{n}}(),
                        Self::Str(s) => s.parse().ok(),
                        _ => None
                    }
                }
            }
        }
    }
}

gen_type_to_num_other!();

#[crabtime::function]
fn gen_type_to_bool() {
    let types = ["I32", "I64", "IBig", "F32", "F64"];
    let arms = types
        .map(|t| {
            let n = t.to_lowercase();
            crabtime::quote! {
                EuType::{{t}}(n) => !n.is_zero(),
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
                    EuType::Vec(ts) => !ts.is_empty(),
                    EuType::Map(kvs) => !kvs.is_empty(),
                    EuType::Set(ts) => !ts.is_empty(),
                    EuType::Expr(ts) => !ts.is_empty(),
                    EuType::Seq(it) => Iterator::peekable(it).peek().is_some(),
                }
            }
        }
    }
}

gen_type_to_bool!();
