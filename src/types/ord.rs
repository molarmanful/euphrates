use std::cmp::Ordering;

use crate::types::EuType;

impl EuType<'_> {
    #[must_use]
    pub fn eqv_ord(&self, other: &Self) -> Ordering {
        self.enum_index().cmp(&other.enum_index())
    }

    fn enum_index(&self) -> u8 {
        match self {
            Self::Bool(_) => 0,
            Self::Char(_) => 1,
            Self::I32(_) => 2,
            Self::I64(_) => 3,
            Self::IBig(_) => 4,
            Self::F64(_) => 5,
            Self::Word(_) => 6,
            Self::Str(_) => 7,
            Self::Opt(_) => 8,
            Self::Res(_) => 9,
            Self::Expr(_) => 10,
            Self::Vec(_) => 11,
            Self::Map(_) => 12,
            Self::Set(_) => 13,
            Self::Seq(_) => 14,
        }
    }

    #[must_use]
    pub fn loose_eq(&self, other: &Self) -> bool {
        if let Some((a, b)) = self.clone().num_tower(other.clone()) {
            a == b
        } else {
            self == other
        }
    }

    #[must_use]
    pub fn loose_cmp(&self, other: &Self) -> Ordering {
        if let Some((a, b)) = self.clone().num_tower(other.clone()) {
            a.cmp(&b)
        } else {
            self.cmp(other)
        }
    }
}

#[crabtime::function]
fn gen_partial_eq() {
    let types = [
        "Bool", "I32", "I64", "IBig", "F64", "Char", "Str", "Word", "Opt", "Res", "Vec", "Map",
        "Set", "Expr",
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
    let types = [
        "Bool", "I32", "I64", "IBig", "F64", "Char", "Str", "Word", "Opt", "Res", "Vec", "Map",
        "Set", "Expr",
    ];
    let arms = types
        .map(|t| {
            crabtime::quote! {
                (Self::{{t}}(l0), Self::{{t}}(r0)) => l0.cmp(r0),
            }
        })
        .join("");

    crabtime::output! {
        impl Ord for EuType<'_> {
            fn cmp(&self, other: &Self) -> Ordering {
                match (self, other) {
                    {{arms}}
                    (Self::Seq(l0), Self::Seq(r0)) => l0.clone().cmp(r0.clone()),
                    (Self::Bool(l0), _) => l0.cmp(&!l0),
                    (_, Self::Bool(r0)) => r0.cmp(&!r0).reverse(),
                    (Self::Word(_), _) => Ordering::Greater,
                    (l0, r0 @ Self::Word(_)) => r0.cmp(l0).reverse(),
                    (a, b) if a.is_int() && b.is_int() => {
                        a.to_ibig().unwrap().cmp(&b.to_ibig().unwrap()).then_with(|| a.eqv_ord(b))
                    }
                    (a, b) if a.is_vecz() || a.is_str() || a.is_expr() || b.is_vecz() || b.is_str() || b.is_expr() => {
                        a.clone().to_seq().cmp(b.clone().to_seq()).then_with(|| a.eqv_ord(b))
                    }
                    (a, b) => a.eqv_ord(b),
                }
            }
        }
    }
}

gen_ord!();

#[cfg(test)]
mod tests {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{
            Hash,
            Hasher,
        },
    };

    use super::*;

    #[test]
    fn ord_eq_consistent() {
        let vs = sample_values();
        for a in &vs {
            for b in &vs {
                assert_consistent(a, b);
            }
        }
    }

    #[test]
    fn ord_antisymmetric() {
        let vs = sample_values();
        for a in &vs {
            for b in &vs {
                assert_eq!(a.cmp(b).reverse(), b.cmp(a));
            }
        }
    }

    #[test]
    fn ints_cmp_properly() {
        assert!(EuType::i64(0) < EuType::i32(1));
        assert!(EuType::i32(2) > EuType::i64(1));
        assert!(EuType::ibig(0) < EuType::i32(1));
        assert!(EuType::i32(100) < EuType::ibig(200));
        assert!(EuType::i64(-5) < EuType::i32(0));
    }

    #[test]
    fn loose_eq() {
        assert!(EuType::i32(1).loose_eq(&EuType::i64(1)));
        assert!(EuType::i32(1).loose_eq(&EuType::ibig(1)));
        assert!(EuType::i32(1).loose_eq(&EuType::f64(1.0)));
        assert!(EuType::ibig(1).loose_eq(&EuType::f64(1.0)));
        assert!(EuType::Bool(true).loose_eq(&EuType::i32(1)));
        assert!(EuType::Bool(false).loose_eq(&EuType::i32(0)));
        assert!(EuType::Bool(true).loose_eq(&EuType::char(1)));
        assert!(EuType::Bool(false).loose_eq(&EuType::char(0)));
        assert!(!EuType::i32(1).loose_eq(&EuType::str("1")));
    }

    #[test]
    fn loose_cmp() {
        assert_eq!(EuType::i32(1).loose_cmp(&EuType::f64(2.0)), Ordering::Less);
        assert_eq!(
            EuType::f64(2.0).loose_cmp(&EuType::i32(1)),
            Ordering::Greater
        );
        assert_eq!(EuType::i32(1).loose_cmp(&EuType::f64(1.0)), Ordering::Equal);
    }

    fn assert_consistent<'a>(a: &EuType<'a>, b: &EuType<'a>) {
        let cmp = a.cmp(b);
        let eq = a == b;
        assert_eq!(cmp == Ordering::Equal, eq);
        if eq {
            assert_eq!(hash(a), hash(b));
        }
    }

    fn hash(t: &EuType<'_>) -> u64 {
        let mut h = DefaultHasher::new();
        t.hash(&mut h);
        h.finish()
    }

    fn sample_values() -> Vec<EuType<'static>> {
        vec![
            EuType::Bool(false),
            EuType::Bool(true),
            EuType::char(0),
            EuType::char(1),
            EuType::i32(-1),
            EuType::i32(0),
            EuType::i32(1),
            EuType::i64(-1),
            EuType::i64(0),
            EuType::i64(1),
            EuType::ibig(-1),
            EuType::ibig(0),
            EuType::ibig(1),
            EuType::f64(-1.0),
            EuType::f64(0.0),
            EuType::f64(1.0),
            EuType::char('a'),
            EuType::str("asdf"),
            EuType::word("asdf"),
            EuType::vec([EuType::i32(1)]),
        ]
    }
}
