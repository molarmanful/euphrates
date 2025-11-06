use ecow::eco_vec;
use winnow::error::{
    ContextError,
    ParseError,
};

use super::*;

#[test]
fn test_empty() {
    assert_eq!(parse(""), Ok(eco_vec![]));
    assert_eq!(parse(" "), Ok(eco_vec![]));
    assert_eq!(parse("\t \n "), Ok(eco_vec![]));
}

#[test]
fn test_int() {
    assert_eq!(parse("1234"), Ok(eco_vec![EuType::ibig(1234).into()]));
    assert_eq!(parse("1234i64"), Ok(eco_vec![EuType::i64(1234).into()]));
}

#[test]
fn test_float() {
    assert_eq!(parse("1234.5"), Ok(eco_vec![EuType::f64(1234.5).into()]));
    assert_eq!(parse("0.1234"), Ok(eco_vec![EuType::f64(0.1234).into()]));
    assert_eq!(parse("1234.0"), Ok(eco_vec![EuType::f64(1234.0).into()]));
    assert_eq!(parse("123e4"), Ok(eco_vec![EuType::f64(123e4).into()]));
    assert_eq!(parse("123e-4"), Ok(eco_vec![EuType::f64(123e-4).into()]));
    assert_eq!(parse("123.0e4"), Ok(eco_vec![EuType::f64(123e4).into()]));
    assert_eq!(parse("12.34e5"), Ok(eco_vec![EuType::f64(12.34e5).into()]));
    assert_eq!(parse("123f32"), Ok(eco_vec![EuType::f32(123.0).into()]));
    assert_eq!(parse("123.0f32"), Ok(eco_vec![EuType::f32(123.0).into()]));
    assert_eq!(parse("123e-4f32"), Ok(eco_vec![EuType::f32(123e-4).into()]));
}

#[test]
fn test_float_invalid() {
    assert!(parse("123e4i32").is_err());
}

#[test]
fn test_word() {
    assert_eq!(parse("asdf"), Ok(eco_vec![EuType::word("asdf").into()]));
    assert_eq!(
        parse("asdf1234"),
        Ok(eco_vec![EuType::word("asdf1234").into()])
    );
}

#[test]
fn test_var() {
    assert_eq!(parse("$asdf"), Ok(eco_vec![EuSyn::Var("asdf".into())]));
    assert_eq!(parse("$"), Ok(eco_vec![EuType::word("$").into()]));
}

#[test]
fn test_dec() {
    assert_eq!(parse(".1234"), Ok(eco_vec![EuType::word(".1234").into()]));
    assert_eq!(
        parse("1234.5.678"),
        Ok(eco_vec![
            EuType::f64(1234.5).into(),
            EuType::word(".678").into()
        ])
    );
    assert_eq!(
        parse(".1234.5.678"),
        Ok(eco_vec![EuType::word(".1234.5.678").into()])
    );
    assert_eq!(
        parse("1234..5678"),
        Ok(eco_vec![
            EuType::ibig(1234).into(),
            EuType::word("..5678").into()
        ])
    );
    assert_eq!(
        parse("123.f32"),
        Ok(eco_vec![
            EuType::ibig(123).into(),
            EuType::word(".f32").into()
        ])
    );
}

#[test]
fn test_str() {
    assert_eq!(
        parse(r#""testing testing 123""#),
        Ok(eco_vec![EuType::str("testing testing 123").into()])
    );
    assert_eq!(
        parse(r#""testing testing 123"#),
        Ok(eco_vec![EuType::str("testing testing 123").into()])
    );
    assert_eq!(
        parse(r#""asdf \" 123""#),
        Ok(eco_vec![EuType::str("asdf \" 123").into()])
    );
    assert_eq!(
        parse(r#""asdf 123 \""#),
        Ok(eco_vec![EuType::str("asdf 123 \"").into()])
    );
    assert_eq!(
        parse(r#""asdf 123 \\""#),
        Ok(eco_vec![EuType::str("asdf 123 \\").into()])
    );
    assert_eq!(parse(r#""\n""#), Ok(eco_vec![EuType::str("\n").into()]));
    assert_eq!(
        parse(r#""\x5a\xff""#),
        Ok(eco_vec![EuType::str("\x5a\u{ff}").into()])
    );
    assert_eq!(
        parse(r#""\u{5}\u{ff}\u{321ab}""#),
        Ok(eco_vec![EuType::str("\u{5}\u{ff}\u{321ab}").into()])
    );
    assert_eq!(
        parse(r#""\u{ff""#),
        Ok(eco_vec![EuType::str("\u{ff}").into()])
    );
}

#[test]
fn test_str_raw() {
    assert_eq!(
        parse("`testing testing 123`"),
        Ok(eco_vec![EuType::str("testing testing 123").into()])
    );
    assert_eq!(
        parse("`testing testing 123"),
        Ok(eco_vec![EuType::str("testing testing 123").into()])
    );
    assert_eq!(
        parse(r#"`asdf \n 123`"#),
        Ok(eco_vec![EuType::str(r#"asdf \n 123"#).into()])
    );
}

#[test]
fn test_str_invalid() {
    assert!(is_err(r#""\"#));
    assert!(is_err(r#""\x1""#));
    assert!(is_err(r#""\x1x""#));
    assert!(is_err(r#""\ux""#));
    assert!(is_err(r#""\u{""#));
    assert!(is_err(r#""\u}""#));
    assert!(is_err(r#""\u{}""#));
    assert!(is_err(r#""\u{ffffff}""#));
    assert!(is_err(r#""\u{dddd}""#));
}

#[test]
fn test_char() {
    assert_eq!(parse("'a"), Ok(eco_vec![EuType::char('a').into()]));
    assert_eq!(parse(r#"'\n"#), Ok(eco_vec![EuType::char('\n').into()]));
    assert_eq!(parse("''"), Ok(eco_vec![EuType::char('\'').into()]));
}

#[test]
fn test_char_invalid() {
    assert!(is_err(r#"'"#));
}

#[test]
fn test_expr() {
    assert_eq!(
        parse(r#"(1 "2" 3+ asdf)"#),
        Ok(eco_vec![
            EuType::expr([
                EuType::ibig(1).into(),
                EuType::str("2").into(),
                EuType::ibig(3).into(),
                EuType::word("+").into(),
                EuType::word("asdf").into()
            ])
            .into()
        ])
    );
    assert_eq!(
        parse(r#"(1 "2" 3+ asdf"#),
        Ok(eco_vec![
            EuType::expr([
                EuType::ibig(1).into(),
                EuType::str("2").into(),
                EuType::ibig(3).into(),
                EuType::word("+").into(),
                EuType::word("asdf").into()
            ])
            .into()
        ])
    );
    assert_eq!(
        parse(r#"((1 "2") 3+ (asdf))"#),
        Ok(eco_vec![
            EuType::expr([
                EuType::expr([EuType::ibig(1).into(), EuType::str("2").into()]).into(),
                EuType::ibig(3).into(),
                EuType::word("+").into(),
                EuType::expr([EuType::word("asdf").into()]).into()
            ])
            .into()
        ])
    );
}

#[test]
fn test_expr_invalid() {
    assert!(is_err(")"));
    assert!(is_err("())asdf"));
}

#[test]
fn test_vec() {
    assert_eq!(
        parse(r#"[1 "2" 3+ asdf]"#),
        Ok(eco_vec![EuSyn::Vec(eco_vec![
            EuType::ibig(1).into(),
            EuType::str("2").into(),
            EuType::ibig(3).into(),
            EuType::word("+").into(),
            EuType::word("asdf").into()
        ])])
    );
    assert_eq!(
        parse(r#"[1 "2" 3+ asdf"#),
        Ok(eco_vec![EuSyn::Vec(eco_vec![
            EuType::ibig(1).into(),
            EuType::str("2").into(),
            EuType::ibig(3).into(),
            EuType::word("+").into(),
            EuType::word("asdf").into()
        ])])
    );
    assert_eq!(
        parse(r#"[[1 "2"] 3+ [asdf]]"#),
        Ok(eco_vec![EuSyn::Vec(eco_vec![
            EuSyn::Vec(eco_vec![EuType::ibig(1).into(), EuType::str("2").into()]),
            EuType::ibig(3).into(),
            EuType::word("+").into(),
            EuSyn::Vec(eco_vec![EuType::word("asdf").into()])
        ])])
    );
}

#[test]
fn test_vec_invalid() {
    assert!(is_err("]"));
    assert!(is_err("[]]asdf"));
}

#[test]
fn test_all() {
    assert_eq!(
        parse(r#"1234"testing testing 123"#),
        Ok(eco_vec![
            EuType::ibig(1234).into(),
            EuType::str("testing testing 123").into()
        ])
    );
    assert_eq!(
        parse(r#"asdf"testing testing 123"#),
        Ok(eco_vec![
            EuType::word("asdf").into(),
            EuType::str("testing testing 123").into()
        ])
    );
    assert_eq!(
        parse("1234e5asdf"),
        Ok(eco_vec![
            EuType::f64(1234e5).into(),
            EuType::word("asdf").into()
        ])
    );
    assert_eq!(
        parse("1234i32test"),
        Ok(eco_vec![
            EuType::i32(1234).into(),
            EuType::word("test").into()
        ])
    );
    assert_eq!(
        parse("123ever"),
        Ok(eco_vec![
            EuType::ibig(123).into(),
            EuType::word("ever").into()
        ])
    );
    assert_eq!(
        parse("123e.4"),
        Ok(eco_vec![
            EuType::ibig(123).into(),
            EuType::word("e.4").into()
        ])
    );
    assert_eq!(
        parse("(1 2+)map"),
        Ok(eco_vec![
            EuType::expr([
                EuType::ibig(1).into(),
                EuType::ibig(2).into(),
                EuType::word("+").into()
            ])
            .into(),
            EuType::word("map").into()
        ])
    );
    assert_eq!(
        parse("12e3.4"),
        Ok(eco_vec![
            EuType::f64(12e3).into(),
            EuType::word(".4").into()
        ])
    );
}

fn parse(input: &str) -> Result<EcoVec<EuSyn<'_>>, ParseError<&str, ContextError>> {
    euphrates.parse(input)
}

fn is_err(input: &str) -> bool {
    euphrates.parse(input).is_err()
}
