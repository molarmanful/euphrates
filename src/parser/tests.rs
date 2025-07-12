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
        Ok(("", vec![EuI64(1234).into()]))
    );
    assert_eq!(
        euphrates.parse_peek("1234u32"),
        Ok(("", vec![EuU32(1234).into()]))
    );
    assert_eq!(
        euphrates.parse_peek("1234isize"),
        Ok(("", vec![EuIsize(1234).into()]))
    );
}

#[test]
fn test_float() {
    assert_eq!(
        euphrates.parse_peek("1234.5"),
        Ok(("", vec![EuF64(1234.5).into()]))
    );
    assert_eq!(
        euphrates.parse_peek(".1234"),
        Ok(("", vec![EuF64(0.1234).into()]))
    );
    assert_eq!(
        euphrates.parse_peek("1234.0"),
        Ok(("", vec![EuF64(1234.0).into()]))
    );
    assert_eq!(
        euphrates.parse_peek("123e4"),
        Ok(("", vec![EuF64(123e4).into()]))
    );
    assert_eq!(
        euphrates.parse_peek("123e-4"),
        Ok(("", vec![EuF64(123e-4).into()]))
    );
    assert_eq!(
        euphrates.parse_peek("123.e4"),
        Ok(("", vec![EuF64(123e4).into()]))
    );
    assert_eq!(
        euphrates.parse_peek("12.34e5"),
        Ok(("", vec![EuF64(12.34e5).into()]))
    );
    assert_eq!(
        euphrates.parse_peek("123f32"),
        Ok(("", vec![EuF32(123.0).into()]))
    );
    assert_eq!(
        euphrates.parse_peek("123.f32"),
        Ok(("", vec![EuF32(123.0).into()]))
    );
    assert_eq!(
        euphrates.parse_peek("123e-4f32"),
        Ok(("", vec![EuF32(123e-4).into()]))
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
        Ok(("", vec![EuI64(1234).into(), EuI64(5678).into()]))
    );
    assert_eq!(
        euphrates.parse_peek("1234.5.678"),
        Ok(("", vec![EuF64(1234.5).into(), EuF64(0.678).into()]))
    );
    assert_eq!(
        euphrates.parse_peek(".1234.5.678"),
        Ok((
            "",
            vec![EuF64(0.1234).into(), EuF64(0.5).into(), EuF64(0.678).into()]
        ))
    );
    assert_eq!(
        euphrates.parse_peek("1234..5678"),
        Ok(("", vec![EuF64(1234.0).into(), EuF64(0.5678).into()]))
    );
    assert_eq!(
        euphrates.parse_peek("12e3.4"),
        Ok(("", vec![EuF64(12e3).into(), EuF64(0.4).into()]))
    );
}

#[test]
fn test_word() {
    assert_eq!(
        euphrates.parse_peek("asdf"),
        Ok(("", vec![EuWord::from("asdf").into()]))
    );
    assert_eq!(
        euphrates.parse_peek("asdf1234"),
        Ok(("", vec![EuWord::from("asdf1234").into()]))
    );
}

#[test]
fn test_dec() {
    assert_eq!(
        euphrates.parse_peek(".1234"),
        Ok(("", vec![EuF64(0.1234).into()]))
    );
    assert_eq!(
        euphrates.parse_peek(".asdf"),
        Ok(("", vec![EuWord::from(".asdf").into()]))
    );
    assert_eq!(
        euphrates.parse_peek("..1234"),
        Ok(("", vec![EuWord::from("..1234").into()]))
    );
}

#[test]
fn test_str() {
    assert_eq!(
        euphrates.parse_peek(r#""testing testing 123""#),
        Ok(("", vec![EuStr::from("testing testing 123").into()]))
    );
    assert_eq!(
        euphrates.parse_peek(r#""testing testing 123"#),
        Ok(("", vec![EuStr::from("testing testing 123").into()]))
    );
    assert_eq!(
        euphrates.parse_peek(r#""asdf \" 123""#),
        Ok(("", vec![EuStr::from("asdf \" 123").into()]))
    );
    assert_eq!(
        euphrates.parse_peek(r#""asdf 123 \""#),
        Ok(("", vec![EuType::Str("asdf 123 \"".into())]))
    );
    assert_eq!(
        euphrates.parse_peek(r#""asdf 123 \\""#),
        Ok(("", vec![EuStr::from("asdf 123 \\").into()]))
    );
    assert_eq!(
        euphrates.parse_peek(r#""\n""#),
        Ok(("", vec![EuStr::from("\n").into()]))
    );
    assert_eq!(
        euphrates.parse_peek(r#""\x5a\xff""#),
        Ok(("", vec![EuStr::from("\x5a\u{ff}").into()]))
    );
    assert_eq!(
        euphrates.parse_peek(r#""\u{5}\u{ff}\u{321ab}""#),
        Ok(("", vec![EuStr::from("\u{5}\u{ff}\u{321ab}").into()]))
    );
    assert_eq!(
        euphrates.parse_peek(r#""\u{ff""#),
        Ok(("", vec![EuStr::from("\u{ff}").into()]))
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
        Ok(("", vec![EuChar('a').into()]))
    );
    assert_eq!(
        euphrates.parse_peek(r#"'\n"#),
        Ok(("", vec![EuChar('\n').into()]))
    );
    assert_eq!(
        euphrates.parse_peek("''"),
        Ok(("", vec![EuChar('\'').into()]))
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
            vec![
                EuI64(1234).into(),
                EuStr::from("testing testing 123").into()
            ]
        ))
    );
    assert_eq!(
        euphrates.parse_peek(r#"asdf"testing testing 123"#),
        Ok((
            "",
            vec![
                EuWord::from("asdf").into(),
                EuStr::from("testing testing 123").into()
            ]
        ))
    );
    assert_eq!(
        euphrates.parse_peek("1234e5asdf"),
        Ok(("", vec![EuF64(1234e5).into(), EuWord::from("asdf").into()]))
    );
    assert_eq!(
        euphrates.parse_peek("1234isizetest"),
        Ok(("", vec![EuIsize(1234).into(), EuWord::from("test").into()]))
    );
    assert_eq!(
        euphrates.parse_peek("123ever"),
        Ok(("", vec![EuI64(123).into(), EuWord::from("ever").into()]))
    );
    assert_eq!(
        euphrates.parse_peek("123e.4"),
        Ok(("", vec![EuI64(123).into(), EuWord::from("e.4").into()]))
    );
}
