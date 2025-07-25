use anyhow::anyhow;
use derive_more::{
    Debug,
    IsVariant,
};
use ecow::{
    EcoVec,
    eco_vec,
};
use hipstr::HipStr;
use num_traits::ToPrimitive;
use winnow::Parser;

use crate::parser::euphrates;

#[derive(Debug, PartialEq, Clone, IsVariant)]
pub enum EuType<'eu> {
    #[debug("{}", if *_0 { "True" } else { "False" })]
    Bool(bool),
    #[debug("{_0:?}isize")]
    Isize(isize),
    #[debug("{_0:?}usize")]
    Usize(usize),
    #[debug("{_0:?}")]
    I32(i32),
    #[debug("{_0:?}u32")]
    U32(u32),
    #[debug("{_0:?}f32")]
    F32(f32),
    #[debug("{_0:?}i64")]
    I64(i64),
    #[debug("{_0:?}u64")]
    U64(u64),
    #[debug("{_0:?}")]
    F64(f64),
    #[debug("{_0:?}i128")]
    I128(i128),
    #[debug("{_0:?}u128")]
    U128(u128),
    #[debug("{_0:?}")]
    Char(char),

    #[debug("{_0:?}")]
    Str(HipStr<'eu>),
    #[debug("{_0}")]
    Word(HipStr<'eu>),

    #[debug("{}", if let Some(t) = _0 { format!("Some:{t:?}") } else { "None".into() })]
    Opt(Option<Box<EuType<'eu>>>),
    #[debug("{}", match _0 { Ok(t) => format!("Ok:{t:?}"), Err(e) => format!("Err:{e:?}") })]
    Res(Result<Box<EuType<'eu>>, Box<EuType<'eu>>>),

    #[debug("Vec:({})", _0.iter().map(|t| format!("{:?}", t)).collect::<EcoVec<_>>().join(" "))]
    Vec(EcoVec<EuType<'eu>>),
    #[debug("({})", _0.iter().map(|t| format!("{:?}", t)).collect::<EcoVec<_>>().join(" "))]
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

    pub fn to_expr<'e>(self) -> anyhow::Result<EcoVec<Self>> {
        match self {
            EuType::Str(s) => euphrates.parse(&s).map_err(|e| anyhow!(e.into_inner())),
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
