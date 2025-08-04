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
    assert_eq!(parse("1234"), Ok(eco_vec![EuType::I32(1234)]));
    assert_eq!(parse("1234i64"), Ok(eco_vec![EuType::I64(1234)]));
}

#[test]
fn test_float() {
    assert_eq!(parse("1234.5"), Ok(eco_vec![EuType::F64(1234.5)]));
    assert_eq!(parse("0.1234"), Ok(eco_vec![EuType::F64(0.1234)]));
    assert_eq!(parse("1234.0"), Ok(eco_vec![EuType::F64(1234.0)]));
    assert_eq!(parse("123e4"), Ok(eco_vec![EuType::F64(123e4)]));
    assert_eq!(parse("123e-4"), Ok(eco_vec![EuType::F64(123e-4)]));
    assert_eq!(parse("123.0e4"), Ok(eco_vec![EuType::F64(123e4)]));
    assert_eq!(parse("12.34e5"), Ok(eco_vec![EuType::F64(12.34e5)]));
    assert_eq!(parse("123f32"), Ok(eco_vec![EuType::F32(123.0)]));
    assert_eq!(parse("123.0f32"), Ok(eco_vec![EuType::F32(123.0)]));
    assert_eq!(parse("123e-4f32"), Ok(eco_vec![EuType::F32(123e-4)]));
}

#[test]
fn test_float_invalid() {
    assert!(parse("123e4i32").is_err());
}

#[test]
fn test_word() {
    assert_eq!(parse("asdf"), Ok(eco_vec![EuType::word("asdf")]));
    assert_eq!(parse("asdf1234"), Ok(eco_vec![EuType::word("asdf1234")]));
}

#[test]
fn test_dec() {
    assert_eq!(parse(".1234"), Ok(eco_vec![EuType::word(".1234")]));
    assert_eq!(
        parse("1234.5.678"),
        Ok(eco_vec![EuType::F64(1234.5), EuType::word(".678")])
    );
    assert_eq!(
        parse(".1234.5.678"),
        Ok(eco_vec![EuType::word(".1234.5.678")])
    );
    assert_eq!(
        parse("1234..5678"),
        Ok(eco_vec![EuType::I32(1234), EuType::word("..5678")])
    );
    assert_eq!(
        parse("123.f32"),
        Ok(eco_vec![EuType::I32(123), EuType::word(".f32")])
    );
}

#[test]
fn test_str() {
    assert_eq!(
        parse(r#""testing testing 123""#),
        Ok(eco_vec![EuType::str("testing testing 123")])
    );
    assert_eq!(
        parse(r#""testing testing 123"#),
        Ok(eco_vec![EuType::str("testing testing 123")])
    );
    assert_eq!(
        parse(r#""asdf \" 123""#),
        Ok(eco_vec![EuType::str("asdf \" 123")])
    );
    assert_eq!(
        parse(r#""asdf 123 \""#),
        Ok(eco_vec![EuType::str("asdf 123 \"")])
    );
    assert_eq!(
        parse(r#""asdf 123 \\""#),
        Ok(eco_vec![EuType::str("asdf 123 \\")])
    );
    assert_eq!(parse(r#""\n""#), Ok(eco_vec![EuType::str("\n")]));
    assert_eq!(
        parse(r#""\x5a\xff""#),
        Ok(eco_vec![EuType::str("\x5a\u{ff}")])
    );
    assert_eq!(
        parse(r#""\u{5}\u{ff}\u{321ab}""#),
        Ok(eco_vec![EuType::str("\u{5}\u{ff}\u{321ab}")])
    );
    assert_eq!(parse(r#""\u{ff""#), Ok(eco_vec![EuType::str("\u{ff}")]));
}

#[test]
fn test_str_raw() {
    assert_eq!(
        parse("`testing testing 123`"),
        Ok(eco_vec![EuType::str("testing testing 123")])
    );
    assert_eq!(
        parse("`testing testing 123"),
        Ok(eco_vec![EuType::str("testing testing 123")])
    );
    assert_eq!(
        parse(r#"`asdf \n 123`"#),
        Ok(eco_vec![EuType::str(r#"asdf \n 123"#)])
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
    assert_eq!(parse("'a"), Ok(eco_vec![EuType::Char('a')]));
    assert_eq!(parse(r#"'\n"#), Ok(eco_vec![EuType::Char('\n')]));
    assert_eq!(parse("''"), Ok(eco_vec![EuType::Char('\'')]));
}

#[test]
fn test_char_invalid() {
    assert!(is_err(r#"'"#));
}

#[test]
fn test_fn() {
    assert_eq!(
        parse(r#"(1 "2" 3+ asdf)"#),
        Ok(eco_vec![EuType::expr([
            EuType::I32(1),
            EuType::str("2"),
            EuType::I32(3),
            EuType::word("+"),
            EuType::word("asdf")
        ])])
    );
    assert_eq!(
        parse(r#"(1 "2" 3+ asdf"#),
        Ok(eco_vec![EuType::expr([
            EuType::I32(1),
            EuType::str("2"),
            EuType::I32(3),
            EuType::word("+"),
            EuType::word("asdf")
        ])])
    );
    assert_eq!(
        parse(r#"((1 "2") 3+ (asdf))"#),
        Ok(eco_vec![EuType::expr([
            EuType::expr([EuType::I32(1), EuType::str("2")]),
            EuType::I32(3),
            EuType::word("+"),
            EuType::expr([EuType::word("asdf")])
        ])])
    );
}

#[test]
fn test_fn_invalid() {
    assert!(is_err(")"));
    assert!(is_err("())asdf"));
}

#[test]
fn test_all() {
    assert_eq!(
        parse(r#"1234"testing testing 123"#),
        Ok(eco_vec![
            EuType::I32(1234),
            EuType::str("testing testing 123")
        ])
    );
    assert_eq!(
        parse(r#"asdf"testing testing 123"#),
        Ok(eco_vec![
            EuType::word("asdf"),
            EuType::str("testing testing 123")
        ])
    );
    assert_eq!(
        parse("1234e5asdf"),
        Ok(eco_vec![EuType::F64(1234e5), EuType::word("asdf")])
    );
    assert_eq!(
        parse("1234i32test"),
        Ok(eco_vec![EuType::I32(1234), EuType::word("test")])
    );
    assert_eq!(
        parse("123ever"),
        Ok(eco_vec![EuType::I32(123), EuType::word("ever")])
    );
    assert_eq!(
        parse("123e.4"),
        Ok(eco_vec![EuType::I32(123), EuType::word("e.4")])
    );
    assert_eq!(
        parse("(1 2+)map"),
        Ok(eco_vec![
            EuType::expr([EuType::I32(1), EuType::I32(2), EuType::word("+")]),
            EuType::word("map")
        ])
    );
    assert_eq!(
        parse("12e3.4"),
        Ok(eco_vec![EuType::F64(12e3), EuType::word(".4")])
    );
}

fn parse(input: &str) -> Result<EcoVec<EuType<'_>>, ParseError<&str, ContextError>> {
    euphrates.parse(input)
}

fn is_err(input: &str) -> bool {
    euphrates.parse(input).is_err()
}
