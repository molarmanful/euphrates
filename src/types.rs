use std::ops::{
    Add,
    Div,
    Mul,
    Neg,
    Sub,
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

use crate::parser::euphrates;

#[derive(Debug, Display, PartialEq, Clone, IsVariant)]
#[display("{_0}")]
pub enum EuType<'eu> {
    #[debug("{}", if *_0 { "True" } else { "False" })]
    Bool(bool),
    #[debug("{_0:?}")]
    I32(i32),
    #[debug("{_0:?}i64")]
    I64(i64),
    #[debug("{_0:?}i128")]
    I128(i128),
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
}

impl EuType<'_> {
    pub fn to_vec(self) -> EcoVec<Self> {
        match self {
            EuType::Vec(ts) | EuType::Expr(ts) => ts,
            EuType::Str(s) => s.chars().map(EuType::Char).collect(),
            EuType::Opt(o) => o.into_iter().map(|t| *t).collect(),
            EuType::Res(r) => r.into_iter().map(|t| *t).collect(),
            _ => eco_vec![self],
        }
    }

    pub fn to_expr(self) -> anyhow::Result<EcoVec<Self>> {
        match self {
            EuType::Str(s) => euphrates.parse(&s).map_err(|e| anyhow!(e.into_inner())),
            _ if self.is_vecz() => Ok(eco_vec![self]),
            _ => Ok(self.to_vec()),
        }
    }

    pub fn is_vecz(&self) -> bool {
        self.is_opt() || self.is_res() || self.is_vec()
    }

    pub fn vecz_map(self, mut f: impl FnMut(Self) -> Self) -> Self {
        match self {
            Self::Opt(o) => Self::Opt(o.map(|t| Box::new(f(*t)))),
            Self::Res(r) => Self::Res(r.map(|t| Box::new(f(*t)))),
            Self::Vec(v) => Self::Vec(v.into_iter().map(f).collect()),
            _ => f(self),
        }
    }

    pub fn vecz_zip(self, t: Self, f: fn(Self, Self) -> Self) -> Self {
        match (self, t) {
            (Self::Opt(a), Self::Opt(b)) => Self::Opt(a.zip(b).map(|(a, b)| Box::new(f(*a, *b)))),
            (Self::Res(Ok(a)), Self::Res(Ok(b))) => Self::Res(Ok(Box::new(f(*a, *b)))),
            (Self::Res(a), Self::Res(b)) => Self::Res(a.and(b)),
            (Self::Vec(a), Self::Vec(b)) => {
                Self::Vec(a.into_iter().zip(b).map(|(a, b)| f(a, b)).collect())
            }
            (Self::Opt(a), b) => Self::Opt(a.map(|t| Box::new(f(*t, b)))),
            (Self::Res(a), b) => Self::Res(a.map(|t| Box::new(f(*t, b)))),
            (a @ Self::Vec(_), b) => a.vecz_map(|t| f(t, b.clone())),
            (a, Self::Opt(b)) => Self::Opt(b.map(|t| Box::new(f(a, *t)))),
            (a, Self::Res(b)) => Self::Res(b.map(|t| Box::new(f(a, *t)))),
            (a, b @ Self::Vec(_)) => b.vecz_map(|t| f(a.clone(), t)),
            (a, b) => f(a, b),
        }
    }
}

#[crabtime::function]
fn gen_fn_to_num() {
    let types = ["I32", "I64", "I128", "F32", "F64"];
    for t in types {
        let tl = t.to_lowercase();
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
                    self.to_{{tl}}().ok_or(anyhow!("{{tl}} conversion failed"))
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
fn gen_fn_to_bool() {
    let types = ["I32", "I64", "I128", "F32", "F64"];
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
                    EuType::Char(_) => true,
                    EuType::Str(s) => s != "",
                    EuType::Word(_) => true,
                    EuType::Opt(o) => o.is_some(),
                    EuType::Res(r) => r.is_ok(),
                    EuType::Vec(ts) | EuType::Expr(ts) => !ts.is_empty(),
                }
            }
        }
    }
}

gen_fn_to_bool!();

#[crabtime::function]
fn gen_fn_neg() {
    let types = ["I32", "I64", "I128", "F32", "F64"];
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
                    _ if self.is_vecz() => self.vecz_map(Self::neg),
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

    let types0 = ["I32", "I64", "I128", "F32", "F64"];
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
                            Some(Box::new(EuType::F64(a + b)))
                        })()),
                        {{arms_parse}}
                        (a, b) if a.is_vecz() || b.is_vecz() => a.vecz_zip(b, Self::{{f}}),
                        _ => EuType::Opt(None),
                    }
                }
            }
        }
    }
}

gen_fn_math_binops!();
