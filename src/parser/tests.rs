use winnow::error::{
    ContextError,
    ParseError,
};

use super::*;

#[test]
fn test_empty() {
    assert_eq!(parse(""), Ok(EuVec::from([])));
    assert_eq!(parse(" "), Ok(EuVec::from([])));
    assert_eq!(parse("\t \n "), Ok(EuVec::from([])));
}

#[test]
fn test_int() {
    assert_eq!(parse("1234"), Ok(EuVec::from([EuI64(1234).into()])));
    assert_eq!(parse("1234u32"), Ok(EuVec::from([EuU32(1234).into()])));
    assert_eq!(parse("1234isize"), Ok(EuVec::from([EuIsize(1234).into()])));
}

#[test]
fn test_float() {
    assert_eq!(parse("1234.5"), Ok(EuVec::from([EuF64(1234.5).into()])));
    assert_eq!(parse(".1234"), Ok(EuVec::from([EuF64(0.1234).into()])));
    assert_eq!(parse("1234.0"), Ok(EuVec::from([EuF64(1234.0).into()])));
    assert_eq!(parse("123e4"), Ok(EuVec::from([EuF64(123e4).into()])));
    assert_eq!(parse("123e-4"), Ok(EuVec::from([EuF64(123e-4).into()])));
    assert_eq!(parse("123.e4"), Ok(EuVec::from([EuF64(123e4).into()])));
    assert_eq!(parse("12.34e5"), Ok(EuVec::from([EuF64(12.34e5).into()])));
    assert_eq!(parse("123f32"), Ok(EuVec::from([EuF32(123.0).into()])));
    assert_eq!(parse("123.f32"), Ok(EuVec::from([EuF32(123.0).into()])));
    assert_eq!(parse("123e-4f32"), Ok(EuVec::from([EuF32(123e-4).into()])));
}

#[test]
fn test_float_invalid() {
    assert!(parse("123e4usize").is_err());
}

#[test]
fn test_num_and_num() {
    assert_eq!(
        parse("1234 5678"),
        Ok(EuVec::from([EuI64(1234).into(), EuI64(5678).into()]))
    );
    assert_eq!(
        parse("1234.5.678"),
        Ok(EuVec::from([EuF64(1234.5).into(), EuF64(0.678).into()]))
    );
    assert_eq!(
        parse(".1234.5.678"),
        Ok(EuVec::from([
            EuF64(0.1234).into(),
            EuF64(0.5).into(),
            EuF64(0.678).into()
        ]))
    );
    assert_eq!(
        parse("1234..5678"),
        Ok(EuVec::from([EuF64(1234.0).into(), EuF64(0.5678).into()]))
    );
    assert_eq!(
        parse("12e3.4"),
        Ok(EuVec::from([EuF64(12e3).into(), EuF64(0.4).into()]))
    );
}

#[test]
fn test_word() {
    assert_eq!(
        parse("asdf"),
        Ok(EuVec::from([EuWord::from("asdf").into()]))
    );
    assert_eq!(
        parse("asdf1234"),
        Ok(EuVec::from([EuWord::from("asdf1234").into()]))
    );
}

#[test]
fn test_dec() {
    assert_eq!(parse(".1234"), Ok(EuVec::from([EuF64(0.1234).into()])));
    assert_eq!(
        parse(".asdf"),
        Ok(EuVec::from([EuWord::from(".asdf").into()]))
    );
    assert_eq!(
        parse("..1234"),
        Ok(EuVec::from([EuWord::from("..1234").into()]))
    );
}

#[test]
fn test_str() {
    assert_eq!(
        parse(r#""testing testing 123""#),
        Ok(EuVec::from([EuStr::from("testing testing 123").into()]))
    );
    assert_eq!(
        parse(r#""testing testing 123"#),
        Ok(EuVec::from([EuStr::from("testing testing 123").into()]))
    );
    assert_eq!(
        parse(r#""asdf \" 123""#),
        Ok(EuVec::from([EuStr::from("asdf \" 123").into()]))
    );
    assert_eq!(
        parse(r#""asdf 123 \""#),
        Ok(EuVec::from([EuType::Str("asdf 123 \"".into())]))
    );
    assert_eq!(
        parse(r#""asdf 123 \\""#),
        Ok(EuVec::from([EuStr::from("asdf 123 \\").into()]))
    );
    assert_eq!(
        parse(r#""\n""#),
        Ok(EuVec::from([EuStr::from("\n").into()]))
    );
    assert_eq!(
        parse(r#""\x5a\xff""#),
        Ok(EuVec::from([EuStr::from("\x5a\u{ff}").into()]))
    );
    assert_eq!(
        parse(r#""\u{5}\u{ff}\u{321ab}""#),
        Ok(EuVec::from([EuStr::from("\u{5}\u{ff}\u{321ab}").into()]))
    );
    assert_eq!(
        parse(r#""\u{ff""#),
        Ok(EuVec::from([EuStr::from("\u{ff}").into()]))
    );
}

#[test]
fn test_str_raw() {
    assert_eq!(
        parse("`testing testing 123`"),
        Ok(EuVec::from([EuStr::from("testing testing 123").into()]))
    );
    assert_eq!(
        parse("`testing testing 123"),
        Ok(EuVec::from([EuStr::from("testing testing 123").into()]))
    );
    assert_eq!(
        parse(r#"`asdf \n 123`"#),
        Ok(EuVec::from([EuStr::from(r#"asdf \n 123"#).into()]))
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
    assert_eq!(parse("'a"), Ok(EuVec::from([EuChar('a').into()])));
    assert_eq!(parse(r#"'\n"#), Ok(EuVec::from([EuChar('\n').into()])));
    assert_eq!(parse("''"), Ok(EuVec::from([EuChar('\'').into()])));
}

#[test]
fn test_char_invalid() {
    assert!(is_err(r#"'"#));
}

#[test]
fn test_fn() {
    assert_eq!(
        parse(r#"(1 "2" 3+ asdf)"#),
        Ok(EuVec::from([EuVec::from([
            EuI64(1).into(),
            EuStr::from("2").into(),
            EuI64(3).into(),
            EuWord::from("+").into(),
            EuWord::from("asdf").into()
        ])
        .into()]))
    );
    assert_eq!(
        parse(r#"(1 "2" 3+ asdf"#),
        Ok(EuVec::from([EuVec::from([
            EuI64(1).into(),
            EuStr::from("2").into(),
            EuI64(3).into(),
            EuWord::from("+").into(),
            EuWord::from("asdf").into()
        ])
        .into()]))
    );
    assert_eq!(
        parse(r#"((1 "2") 3+ (asdf))"#),
        Ok(EuVec::from([EuVec::from([
            EuVec::from([EuI64(1).into(), EuStr::from("2").into()]).into(),
            EuI64(3).into(),
            EuWord::from("+").into(),
            EuVec::from([EuWord::from("asdf").into()]).into()
        ])
        .into()]))
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
        Ok(EuVec::from([
            EuI64(1234).into(),
            EuStr::from("testing testing 123").into()
        ]))
    );
    assert_eq!(
        parse(r#"asdf"testing testing 123"#),
        Ok(EuVec::from([
            EuWord::from("asdf").into(),
            EuStr::from("testing testing 123").into()
        ]))
    );
    assert_eq!(
        parse("1234e5asdf"),
        Ok(EuVec::from([
            EuF64(1234e5).into(),
            EuWord::from("asdf").into()
        ]))
    );
    assert_eq!(
        parse("1234isizetest"),
        Ok(EuVec::from([
            EuIsize(1234).into(),
            EuWord::from("test").into()
        ]))
    );
    assert_eq!(
        parse("123ever"),
        Ok(EuVec::from([
            EuI64(123).into(),
            EuWord::from("ever").into()
        ]))
    );
    assert_eq!(
        parse("123e.4"),
        Ok(EuVec::from([EuI64(123).into(), EuWord::from("e.4").into()]))
    );
    assert_eq!(
        parse("(1 2+)map"),
        Ok(EuVec::from([
            EuVec::from([EuI64(1).into(), EuI64(2).into(), EuWord::from("+").into()]).into(),
            EuWord::from("map").into()
        ]))
    );
}

fn parse(input: &str) -> Result<EuVec<'_>, ParseError<&str, ContextError>> {
    euphrates.parse(input)
}

fn is_err(input: &str) -> bool {
    euphrates.parse(input).is_err()
}
