use winnow::error::{
    ContextError,
    ParseError,
};

use super::*;

#[test]
fn test_empty() {
    assert_eq!(parse(""), Ok(imbl::vector![]));
    assert_eq!(parse(" "), Ok(imbl::vector![]));
    assert_eq!(parse("\t \n "), Ok(imbl::vector![]));
}

#[test]
fn test_int() {
    assert_eq!(parse("1234"), Ok(imbl::vector![EuType::I32(1234)]));
    assert_eq!(parse("1234i64"), Ok(imbl::vector![EuType::I64(1234)]));
}

#[test]
fn test_float() {
    assert_eq!(parse("1234.5"), Ok(imbl::vector![EuType::F64(1234.5)]));
    assert_eq!(parse("0.1234"), Ok(imbl::vector![EuType::F64(0.1234)]));
    assert_eq!(parse("1234.0"), Ok(imbl::vector![EuType::F64(1234.0)]));
    assert_eq!(parse("123e4"), Ok(imbl::vector![EuType::F64(123e4)]));
    assert_eq!(parse("123e-4"), Ok(imbl::vector![EuType::F64(123e-4)]));
    assert_eq!(parse("123.0e4"), Ok(imbl::vector![EuType::F64(123e4)]));
    assert_eq!(parse("12.34e5"), Ok(imbl::vector![EuType::F64(12.34e5)]));
    assert_eq!(parse("123f32"), Ok(imbl::vector![EuType::F32(123.0)]));
    assert_eq!(parse("123.0f32"), Ok(imbl::vector![EuType::F32(123.0)]));
    assert_eq!(parse("123e-4f32"), Ok(imbl::vector![EuType::F32(123e-4)]));
}

#[test]
fn test_float_invalid() {
    assert!(parse("123e4i32").is_err());
}

#[test]
fn test_word() {
    assert_eq!(
        parse("asdf"),
        Ok(imbl::vector![EuType::Word("asdf".into())])
    );
    assert_eq!(
        parse("asdf1234"),
        Ok(imbl::vector![EuType::Word("asdf1234".into())])
    );
}

#[test]
fn test_dec() {
    assert_eq!(
        parse(".1234"),
        Ok(imbl::vector![EuType::Word(".1234".into())])
    );
    assert_eq!(
        parse("1234.5.678"),
        Ok(imbl::vector![
            EuType::F64(1234.5),
            EuType::Word(".678".into())
        ])
    );
    assert_eq!(
        parse(".1234.5.678"),
        Ok(imbl::vector![EuType::Word(".1234.5.678".into())])
    );
    assert_eq!(
        parse("1234..5678"),
        Ok(imbl::vector![
            EuType::I32(1234),
            EuType::Word("..5678".into())
        ])
    );
    assert_eq!(
        parse("123.f32"),
        Ok(imbl::vector![EuType::I32(123), EuType::Word(".f32".into())])
    );
}

#[test]
fn test_str() {
    assert_eq!(
        parse(r#""testing testing 123""#),
        Ok(imbl::vector![EuType::Str("testing testing 123".into())])
    );
    assert_eq!(
        parse(r#""testing testing 123"#),
        Ok(imbl::vector![EuType::Str("testing testing 123".into())])
    );
    assert_eq!(
        parse(r#""asdf \" 123""#),
        Ok(imbl::vector![EuType::Str("asdf \" 123".into())])
    );
    assert_eq!(
        parse(r#""asdf 123 \""#),
        Ok(imbl::vector![EuType::Str("asdf 123 \"".into())])
    );
    assert_eq!(
        parse(r#""asdf 123 \\""#),
        Ok(imbl::vector![EuType::Str("asdf 123 \\".into())])
    );
    assert_eq!(
        parse(r#""\n""#),
        Ok(imbl::vector![EuType::Str("\n".into())])
    );
    assert_eq!(
        parse(r#""\x5a\xff""#),
        Ok(imbl::vector![EuType::Str("\x5a\u{ff}".into())])
    );
    assert_eq!(
        parse(r#""\u{5}\u{ff}\u{321ab}""#),
        Ok(imbl::vector![EuType::Str("\u{5}\u{ff}\u{321ab}".into())])
    );
    assert_eq!(
        parse(r#""\u{ff""#),
        Ok(imbl::vector![EuType::Str("\u{ff}".into())])
    );
}

#[test]
fn test_str_raw() {
    assert_eq!(
        parse("`testing testing 123`"),
        Ok(imbl::vector![EuType::Str("testing testing 123".into())])
    );
    assert_eq!(
        parse("`testing testing 123"),
        Ok(imbl::vector![EuType::Str("testing testing 123".into())])
    );
    assert_eq!(
        parse(r#"`asdf \n 123`"#),
        Ok(imbl::vector![EuType::Str(r#"asdf \n 123"#.into())])
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
    assert_eq!(parse("'a"), Ok(imbl::vector![EuType::Char('a')]));
    assert_eq!(parse(r#"'\n"#), Ok(imbl::vector![EuType::Char('\n')]));
    assert_eq!(parse("''"), Ok(imbl::vector![EuType::Char('\'')]));
}

#[test]
fn test_char_invalid() {
    assert!(is_err(r#"'"#));
}

#[test]
fn test_fn() {
    assert_eq!(
        parse(r#"(1 "2" 3+ asdf)"#),
        Ok(imbl::vector![EuType::Expr(imbl::vector![
            EuType::I32(1),
            EuType::Str("2".into()),
            EuType::I32(3),
            EuType::Word("+".into()),
            EuType::Word("asdf".into())
        ])])
    );
    assert_eq!(
        parse(r#"(1 "2" 3+ asdf"#),
        Ok(imbl::vector![EuType::Expr(imbl::vector![
            EuType::I32(1),
            EuType::Str("2".into()),
            EuType::I32(3),
            EuType::Word("+".into()),
            EuType::Word("asdf".into())
        ])])
    );
    assert_eq!(
        parse(r#"((1 "2") 3+ (asdf))"#),
        Ok(imbl::vector![EuType::Expr(imbl::vector![
            EuType::Expr(imbl::vector![EuType::I32(1), EuType::Str("2".into())]),
            EuType::I32(3),
            EuType::Word("+".into()),
            EuType::Expr(imbl::vector![EuType::Word("asdf".into())])
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
        Ok(imbl::vector![
            EuType::I32(1234),
            EuType::Str("testing testing 123".into())
        ])
    );
    assert_eq!(
        parse(r#"asdf"testing testing 123"#),
        Ok(imbl::vector![
            EuType::Word("asdf".into()),
            EuType::Str("testing testing 123".into())
        ])
    );
    assert_eq!(
        parse("1234e5asdf"),
        Ok(imbl::vector![
            EuType::F64(1234e5),
            EuType::Word("asdf".into())
        ])
    );
    assert_eq!(
        parse("1234i32test"),
        Ok(imbl::vector![
            EuType::I32(1234),
            EuType::Word("test".into())
        ])
    );
    assert_eq!(
        parse("123ever"),
        Ok(imbl::vector![EuType::I32(123), EuType::Word("ever".into())])
    );
    assert_eq!(
        parse("123e.4"),
        Ok(imbl::vector![EuType::I32(123), EuType::Word("e.4".into())])
    );
    assert_eq!(
        parse("(1 2+)map"),
        Ok(imbl::vector![
            EuType::Expr(imbl::vector![
                EuType::I32(1),
                EuType::I32(2),
                EuType::Word("+".into())
            ]),
            EuType::Word("map".into())
        ])
    );
    assert_eq!(
        parse("12e3.4"),
        Ok(imbl::vector![EuType::F64(12e3), EuType::Word(".4".into())])
    );
}

fn parse(input: &str) -> Result<imbl::Vector<EuType<'_>>, ParseError<&str, ContextError>> {
    euphrates.parse(input)
}

fn is_err(input: &str) -> bool {
    euphrates.parse(input).is_err()
}
