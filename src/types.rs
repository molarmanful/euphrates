use derive_more::{
    Debug,
    IsVariant,
};
use ecow::{
    EcoVec,
    eco_vec,
};
use hipstr::HipStr;
use num_traits::ToPrimitive as _;
use winnow::Parser as _;

use crate::{
    EvalError,
    parser::euphrates,
};

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
    I128(i128),
    U128(u128),
    Char(char),
    Str(HipStr<'eu>),
    Word(HipStr<'eu>),
    Opt(Option<Box<EuType<'eu>>>),
    Res(Result<Box<EuType<'eu>>, Box<EuType<'eu>>>),
    Vec(EcoVec<EuType<'eu>>),
    Expr(EcoVec<EuType<'eu>>),
}

impl EuType<'_> {
    pub fn to_vec(self) -> EcoVec<Self> {
        match self {
            EuType::Vec(ts) => ts,
            EuType::Expr(ts) => ts,
            EuType::Str(s) => s.chars().map(EuType::Char).collect(),
            EuType::Opt(o) => o.into_iter().map(|x| *x).collect(),
            EuType::Res(r) => r.into_iter().map(|x| *x).collect(),
            _ => eco_vec![self],
        }
    }

    pub fn to_expr<'e>(self) -> Result<EcoVec<Self>, EvalError<'e>> {
        match self {
            EuType::Str(s) => euphrates.parse(&s).map_err(|e| e.to_string().into()),
            _ => Ok(self.to_vec()),
        }
    }
}

#[crabtime::function]
fn gen_fn_to_num() {
    let types = [
        "Isize", "Usize", "I32", "U32", "F32", "I64", "U64", "F64", "I128", "U128",
    ];
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
    let types = [
        "Isize", "Usize", "I32", "U32", "F32", "I64", "U64", "F64", "I128", "U128",
    ];
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
                    EuType::Vec(ts) => ts.len() > 0,
                    EuType::Expr(ts) => ts.len() > 0,
                }
            }
        }
    }
}

gen_fn_to_bool!();
