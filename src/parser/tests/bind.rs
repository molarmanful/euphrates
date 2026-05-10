use ecow::eco_vec;

use super::*;
use crate::types::{
    EuBind,
    EuSyn,
};

#[test]
fn base() {
    assert_eq!(parse(r"\[]"), Ok(eco_vec![EuSyn::Bind(eco_vec![])]));
    assert_eq!(
        parse(r"\[a b]"),
        Ok(eco_vec![EuSyn::Bind(eco_vec![
            EuBind::word("a"),
            EuBind::word("b")
        ])])
    );
    assert_eq!(
        parse(r#"\["asdf"]"#),
        Ok(eco_vec![EuSyn::Bind(eco_vec![EuBind::str("asdf")])])
    );
    assert_eq!(
        parse(r"\['a]"),
        Ok(eco_vec![EuSyn::Bind(eco_vec![EuBind::Char('a')])])
    );
    assert_eq!(
        parse(r"\[3]"),
        Ok(eco_vec![EuSyn::Bind(eco_vec![EuBind::ibig(3)])])
    );
    assert_eq!(
        parse(r"\[3f64]"),
        Ok(eco_vec![EuSyn::Bind(eco_vec![EuBind::f64(3)])])
    );
}

#[test]
fn tag() {
    assert_eq!(
        parse(r"\[$None()]"),
        Ok(eco_vec![EuSyn::Bind(eco_vec![EuBind::tag("None", [])])])
    );
    assert_eq!(
        parse(r"\[$None()]"),
        Ok(eco_vec![EuSyn::Bind(eco_vec![EuBind::tag("None", [])])])
    );
}

#[test]
fn bind() {
    assert_eq!(
        parse(r"\[1\a]"),
        Ok(eco_vec![EuSyn::Bind(eco_vec![EuBind::bind(
            EuBind::ibig(1),
            EuBind::word("a"),
        )])])
    );
    assert_eq!(
        parse(r"\[1\a\b]"),
        Ok(eco_vec![EuSyn::Bind(eco_vec![EuBind::bind(
            EuBind::ibig(1),
            EuBind::bind(EuBind::word("a"), EuBind::word("b"))
        )])])
    );
    assert_eq!(
        parse(r"\[1\[2 b]]"),
        Ok(eco_vec![EuSyn::Bind(eco_vec![EuBind::bind(
            EuBind::ibig(1),
            EuBind::vecz([EuBind::ibig(2), EuBind::word("b")])
        )])])
    );
    assert_eq!(
        parse(r"\[(1\a)\b]"),
        Ok(eco_vec![EuSyn::Bind(eco_vec![EuBind::bind(
            EuBind::union([EuBind::bind(EuBind::ibig(1), EuBind::word("a"))]),
            EuBind::word("b")
        )])])
    );
}

#[test]
fn vecz() {
    assert_eq!(
        parse(r"\[[]]"),
        Ok(eco_vec![EuSyn::Bind(eco_vec![EuBind::vecz([])])])
    );
    assert_eq!(
        parse(r"\[[a b]]"),
        Ok(eco_vec![EuSyn::Bind(eco_vec![EuBind::vecz([
            EuBind::word("a"),
            EuBind::word("b")
        ])])])
    );
}

#[test]
fn map() {
    assert_eq!(
        parse(r"\[{}]"),
        Ok(eco_vec![EuSyn::Bind(eco_vec![EuBind::map([])])])
    );
    assert_eq!(
        parse(r"\[{a b}]"),
        Ok(eco_vec![EuSyn::Bind(eco_vec![EuBind::map([
            EuBind::word("a"),
            EuBind::word("b")
        ])])])
    );
}
