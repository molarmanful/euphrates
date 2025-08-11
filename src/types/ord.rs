use std::cmp::Ordering;

use num_traits::{
    AsPrimitive,
    ToPrimitive,
};
use ordered_float::OrderedFloat;

use super::EuType;

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
