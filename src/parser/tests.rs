use super::*;

#[test]
fn test_empty() {
    assert_eq!(euphrates.parse_peek(""), Ok(("", vec![])));
    assert_eq!(euphrates.parse_peek(" "), Ok(("", vec![])));
    assert_eq!(euphrates.parse_peek("\t \n "), Ok(("", vec![])));
}

#[test]
fn test_int() {
    assert_eq!(
        euphrates.parse_peek("1234"),
        Ok(("", vec![EuType::I64(1234)]))
    );
    assert_eq!(
        euphrates.parse_peek("1234u32"),
        Ok(("", vec![EuType::U32(1234)]))
    );
    assert_eq!(
        euphrates.parse_peek("1234isize"),
        Ok(("", vec![EuType::ISize(1234)]))
    );
}

#[test]
fn test_float() {
    assert_eq!(
        euphrates.parse_peek("1234.5"),
        Ok(("", vec![EuType::F64(1234.5)]))
    );
    assert_eq!(
        euphrates.parse_peek(".1234"),
        Ok(("", vec![EuType::F64(0.1234)]))
    );
    assert_eq!(
        euphrates.parse_peek("1234.0"),
        Ok(("", vec![EuType::F64(1234.0)]))
    );
    assert_eq!(
        euphrates.parse_peek("123e4"),
        Ok(("", vec![EuType::F64(123e4)]))
    );
    assert_eq!(
        euphrates.parse_peek("123e-4"),
        Ok(("", vec![EuType::F64(123e-4)]))
    );
    assert_eq!(
        euphrates.parse_peek("123.e4"),
        Ok(("", vec![EuType::F64(123e4)]))
    );
    assert_eq!(
        euphrates.parse_peek("12.34e5"),
        Ok(("", vec![EuType::F64(12.34e5)]))
    );
    assert_eq!(
        euphrates.parse_peek("123f32"),
        Ok(("", vec![EuType::F32(123.0)]))
    );
    assert_eq!(
        euphrates.parse_peek("123.f32"),
        Ok(("", vec![EuType::F32(123.0)]))
    );
    assert_eq!(
        euphrates.parse_peek("123e-4f32"),
        Ok(("", vec![EuType::F32(123e-4)]))
    );
}

#[test]
fn test_float_invalid() {
    assert!(euphrates.parse("123e4usize").is_err());
}

#[test]
fn test_num_and_num() {
    assert_eq!(
        euphrates.parse_peek("1234 5678"),
        Ok(("", vec![EuType::I64(1234), EuType::I64(5678)]))
    );
    assert_eq!(
        euphrates.parse_peek("1234.5.678"),
        Ok(("", vec![EuType::F64(1234.5), EuType::F64(0.678)]))
    );
    assert_eq!(
        euphrates.parse_peek(".1234.5.678"),
        Ok((
            "",
            vec![EuType::F64(0.1234), EuType::F64(0.5), EuType::F64(0.678)]
        ))
    );
    assert_eq!(
        euphrates.parse_peek("1234..5678"),
        Ok(("", vec![EuType::F64(1234.0), EuType::F64(0.5678)]))
    );
    assert_eq!(
        euphrates.parse_peek("12e3.4"),
        Ok(("", vec![EuType::F64(12e3), EuType::F64(0.4)]))
    );
}

#[test]
fn test_word() {
    assert_eq!(
        euphrates.parse_peek("asdf"),
        Ok(("", vec![EuType::Word("asdf".into())]))
    );
    assert_eq!(
        euphrates.parse_peek("asdf1234"),
        Ok(("", vec![EuType::Word("asdf1234".into())]))
    );
}

#[test]
fn test_dec() {
    assert_eq!(
        euphrates.parse_peek(".1234"),
        Ok(("", vec![EuType::F64(0.1234)]))
    );
    assert_eq!(
        euphrates.parse_peek(".asdf"),
        Ok(("", vec![EuType::Word(".asdf".into())]))
    );
    assert_eq!(
        euphrates.parse_peek("..1234"),
        Ok(("", vec![EuType::Word("..1234".into())]))
    );
}

#[test]
fn test_str() {
    assert_eq!(
        euphrates.parse_peek(r#""testing testing 123""#),
        Ok(("", vec![EuType::Str("testing testing 123".into())]))
    );
    assert_eq!(
        euphrates.parse_peek(r#""testing testing 123"#),
        Ok(("", vec![EuType::Str("testing testing 123".into())]))
    );
    assert_eq!(
        euphrates.parse_peek(r#""asdf \" 123""#),
        Ok(("", vec![EuType::Str("asdf \" 123".into())]))
    );
    assert_eq!(
        euphrates.parse_peek(r#""asdf 123 \""#),
        Ok(("", vec![EuType::Str("asdf 123 \"".into())]))
    );
    assert_eq!(
        euphrates.parse_peek(r#""asdf 123 \\""#),
        Ok(("", vec![EuType::Str("asdf 123 \\".into())]))
    );
    assert_eq!(
        euphrates.parse_peek(r#""\n""#),
        Ok(("", vec![EuType::Str("\n".into())]))
    );
    assert_eq!(
        euphrates.parse_peek(r#""\x5a\xff""#),
        Ok(("", vec![EuType::Str("\x5a\u{ff}".into())]))
    );
    assert_eq!(
        euphrates.parse_peek(r#""\u{5}\u{ff}\u{321ab}""#),
        Ok(("", vec![EuType::Str("\u{5}\u{ff}\u{321ab}".into())]))
    );
    assert_eq!(
        euphrates.parse_peek(r#""\u{ff""#),
        Ok(("", vec![EuType::Str("\u{ff}".into())]))
    );
}

#[test]
fn test_str_invalid() {
    assert!(euphrates.parse(r#""\"#).is_err());
    assert!(euphrates.parse(r#""\x1""#).is_err());
    assert!(euphrates.parse(r#""\x1x""#).is_err());
    assert!(euphrates.parse(r#""\ux""#).is_err());
    assert!(euphrates.parse(r#""\u{""#).is_err());
    assert!(euphrates.parse(r#""\u}""#).is_err());
    assert!(euphrates.parse(r#""\u{}""#).is_err());
    assert!(euphrates.parse(r#""\u{ffffff}""#).is_err());
    assert!(euphrates.parse(r#""\u{dddd}""#).is_err());
}

#[test]
fn test_char() {
    assert_eq!(
        euphrates.parse_peek("'a"),
        Ok(("", vec![EuType::Char('a')]))
    );
    assert_eq!(
        euphrates.parse_peek(r#"'\n"#),
        Ok(("", vec![EuType::Char('\n')]))
    );
    assert_eq!(
        euphrates.parse_peek("''"),
        Ok(("", vec![EuType::Char('\'')]))
    );
}

#[test]
fn test_char_invalid() {
    assert!(euphrates.parse(r#"'"#).is_err());
}

#[test]
fn test_all() {
    assert_eq!(
        euphrates.parse_peek(r#"1234"testing testing 123"#),
        Ok((
            "",
            vec![EuType::I64(1234), EuType::Str("testing testing 123".into())]
        ))
    );
    assert_eq!(
        euphrates.parse_peek(r#"asdf"testing testing 123"#),
        Ok((
            "",
            vec![
                EuType::Word("asdf".into()),
                EuType::Str("testing testing 123".into())
            ]
        ))
    );
    assert_eq!(
        euphrates.parse_peek("1234e5asdf"),
        Ok(("", vec![EuType::F64(1234e5), EuType::Word("asdf".into())]))
    );
    assert_eq!(
        euphrates.parse_peek("1234isizetest"),
        Ok(("", vec![EuType::ISize(1234), EuType::Word("test".into())]))
    );
    assert_eq!(
        euphrates.parse_peek("123ever"),
        Ok(("", vec![EuType::I64(123), EuType::Word("ever".into())]))
    );
    assert_eq!(
        euphrates.parse_peek("123e.4"),
        Ok(("", vec![EuType::I64(123), EuType::Word("e.4".into())]))
    );
}
