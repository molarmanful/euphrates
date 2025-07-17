use derive_more::{
    Debug,
    IsVariant,
};
use hipstr::HipStr;
use num_traits::ToPrimitive as _;

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
    Vec(Vec<EuType<'eu>>),
    Expr(Vec<EuType<'eu>>),
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
            fn from(value: EuType<'_>) -> Self {
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
