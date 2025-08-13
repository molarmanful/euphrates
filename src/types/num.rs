use std::ops::{
    Add,
    Div,
    Mul,
    Neg,
    Rem,
    Sub,
};

use anyhow::anyhow;
use num_traits::AsPrimitive;
use ordered_float::OrderedFloat;

use super::{
    EuRes,
    EuType,
};

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

    let arms_ibig = types0
        .map(|t| {
            let n = t.to_lowercase();
            if t.chars().next() == Some('I') {
                crabtime::quote! {
                    (a @ Self::IBig(_), Self::{{t}}(b)) => Some((a, Self::ibig(b))),
                    (Self::{{t}}(a), b @ Self::IBig(_)) => Some((Self::ibig(a), b)),
                }
            } else {
                crabtime::quote! {
                    (Self::IBig(a), b @ Self::{{t}}(_)) => Some((Self::{{n}}(a.to_{{n}}().value()), b)),
                    (a @ Self::{{t}}(_), Self::IBig(b)) => Some((a, Self::{{n}}(b.to_{{n}}().value()))),
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
                    {{arms_ibig}}
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
fn gen_impl_neg() {
    let types = ["I32", "I64", "IBig", "F32", "F64"];
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

    let types = ["I32", "I64", "IBig", "F32", "F64"];
    for name in ["Add", "Sub", "Mul", "Div", "Rem"] {
        let f = name.to_lowercase();
        let fq = format!(r#""{f}""#);

        let arms = types
            .map(|t| {
                if t == "IBig" {
                    let un = if name == "Div" || name == "Rem" {
                        crabtime::quote! {
                            (Self::IBig(a), Self::IBig(b)) if b.is_zero() => {
                                Err(anyhow!("{} on `{a:?}` and `0` is undefined", {{fq}}).into())
                            }
                        }
                    } else {
                        crabtime::quote!()
                    };
                    crabtime::quote! {
                        {{un}}
                        (Self::IBig(a), Self::IBig(b)) => Ok(Self::IBig(a.{{f}}(b))),
                    }
                } else if t.chars().next() == Some('I') {
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

        let arm_mod = types
            .map(|t| {
                if name == "Rem" && t != "IBig" && t.chars().next() == Some('I') {
                    crabtime::quote! {
                        (Self::IBig(a), Self::{{t}}(0)) => {
                            Err(anyhow!("{} on `{a:?}` and `0` is undefined", {{fq}}).into())
                        }
                        (Self::IBig(ref a), Self::{{t}}(b)) => Ok(Self::{{t}}(a.rem(b))),
                    }
                } else {
                    crabtime::quote!()
                }
            })
            .join("");

        crabtime::output! {
            impl {{name}} for EuType<'_> {
                type Output = EuRes<Self>;

                fn {{f}}(self, rhs: Self) -> Self::Output {
                    match (self, rhs) {
                        {{arms}}
                        {{arm_mod}}
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
