use super::*;

#[test]
fn test_empty() {
    assert_eq!(parse(""), Ok(("", EuFn::from([]))));
    assert_eq!(parse(" "), Ok(("", EuFn::from([]))));
    assert_eq!(parse("\t \n "), Ok(("", EuFn::from([]))));
}

#[test]
fn test_int() {
    assert_eq!(parse("1234"), Ok(("", EuFn::from([EuI64(1234).into()]))));
    assert_eq!(parse("1234u32"), Ok(("", EuFn::from([EuU32(1234).into()]))));
    assert_eq!(
        parse("1234isize"),
        Ok(("", EuFn::from([EuIsize(1234).into()])))
    );
}

#[test]
fn test_float() {
    assert_eq!(
        parse("1234.5"),
        Ok(("", EuFn::from([EuF64(1234.5).into()])))
    );
    assert_eq!(parse(".1234"), Ok(("", EuFn::from([EuF64(0.1234).into()]))));
    assert_eq!(
        parse("1234.0"),
        Ok(("", EuFn::from([EuF64(1234.0).into()])))
    );
    assert_eq!(parse("123e4"), Ok(("", EuFn::from([EuF64(123e4).into()]))));
    assert_eq!(
        parse("123e-4"),
        Ok(("", EuFn::from([EuF64(123e-4).into()])))
    );
    assert_eq!(parse("123.e4"), Ok(("", EuFn::from([EuF64(123e4).into()]))));
    assert_eq!(
        parse("12.34e5"),
        Ok(("", EuFn::from([EuF64(12.34e5).into()])))
    );
    assert_eq!(parse("123f32"), Ok(("", EuFn::from([EuF32(123.0).into()]))));
    assert_eq!(
        parse("123.f32"),
        Ok(("", EuFn::from([EuF32(123.0).into()])))
    );
    assert_eq!(
        parse("123e-4f32"),
        Ok(("", EuFn::from([EuF32(123e-4).into()])))
    );
}

#[test]
fn test_float_invalid() {
    assert!(parse("123e4usize").is_err());
}

#[test]
fn test_num_and_num() {
    assert_eq!(
        parse("1234 5678"),
        Ok(("", EuFn::from([EuI64(1234).into(), EuI64(5678).into()])))
    );
    assert_eq!(
        parse("1234.5.678"),
        Ok(("", EuFn::from([EuF64(1234.5).into(), EuF64(0.678).into()])))
    );
    assert_eq!(
        parse(".1234.5.678"),
        Ok((
            "",
            EuFn::from([EuF64(0.1234).into(), EuF64(0.5).into(), EuF64(0.678).into()])
        ))
    );
    assert_eq!(
        parse("1234..5678"),
        Ok(("", EuFn::from([EuF64(1234.0).into(), EuF64(0.5678).into()])))
    );
    assert_eq!(
        parse("12e3.4"),
        Ok(("", EuFn::from([EuF64(12e3).into(), EuF64(0.4).into()])))
    );
}

#[test]
fn test_word() {
    assert_eq!(
        parse("asdf"),
        Ok(("", EuFn::from([EuWord::from("asdf").into()])))
    );
    assert_eq!(
        parse("asdf1234"),
        Ok(("", EuFn::from([EuWord::from("asdf1234").into()])))
    );
}

#[test]
fn test_dec() {
    assert_eq!(parse(".1234"), Ok(("", EuFn::from([EuF64(0.1234).into()]))));
    assert_eq!(
        parse(".asdf"),
        Ok(("", EuFn::from([EuWord::from(".asdf").into()])))
    );
    assert_eq!(
        parse("..1234"),
        Ok(("", EuFn::from([EuWord::from("..1234").into()])))
    );
}

#[test]
fn test_str() {
    assert_eq!(
        parse(r#""testing testing 123""#),
        Ok(("", EuFn::from([EuStr::from("testing testing 123").into()])))
    );
    assert_eq!(
        parse(r#""testing testing 123"#),
        Ok(("", EuFn::from([EuStr::from("testing testing 123").into()])))
    );
    assert_eq!(
        parse(r#""asdf \" 123""#),
        Ok(("", EuFn::from([EuStr::from("asdf \" 123").into()])))
    );
    assert_eq!(
        parse(r#""asdf 123 \""#),
        Ok(("", EuFn::from([EuType::Str("asdf 123 \"".into())])))
    );
    assert_eq!(
        parse(r#""asdf 123 \\""#),
        Ok(("", EuFn::from([EuStr::from("asdf 123 \\").into()])))
    );
    assert_eq!(
        parse(r#""\n""#),
        Ok(("", EuFn::from([EuStr::from("\n").into()])))
    );
    assert_eq!(
        parse(r#""\x5a\xff""#),
        Ok(("", EuFn::from([EuStr::from("\x5a\u{ff}").into()])))
    );
    assert_eq!(
        parse(r#""\u{5}\u{ff}\u{321ab}""#),
        Ok(("", EuFn::from([EuStr::from("\u{5}\u{ff}\u{321ab}").into()])))
    );
    assert_eq!(
        parse(r#""\u{ff""#),
        Ok(("", EuFn::from([EuStr::from("\u{ff}").into()])))
    );
}

#[test]
fn test_str_raw() {
    assert_eq!(
        parse("`testing testing 123`"),
        Ok(("", EuFn::from([EuStr::from("testing testing 123").into()])))
    );
    assert_eq!(
        parse("`testing testing 123"),
        Ok(("", EuFn::from([EuStr::from("testing testing 123").into()])))
    );
    assert_eq!(
        parse(r#"`asdf \n 123`"#),
        Ok(("", EuFn::from([EuStr::from(r#"asdf \n 123"#).into()])))
    );
}

#[test]
fn test_str_invalid() {
    assert!(parse(r#""\"#).is_err());
    assert!(parse(r#""\x1""#).is_err());
    assert!(parse(r#""\x1x""#).is_err());
    assert!(parse(r#""\ux""#).is_err());
    assert!(parse(r#""\u{""#).is_err());
    assert!(parse(r#""\u}""#).is_err());
    assert!(parse(r#""\u{}""#).is_err());
    assert!(parse(r#""\u{ffffff}""#).is_err());
    assert!(parse(r#""\u{dddd}""#).is_err());
}

#[test]
fn test_char() {
    assert_eq!(parse("'a"), Ok(("", EuFn::from([EuChar('a').into()]))));
    assert_eq!(parse(r#"'\n"#), Ok(("", EuFn::from([EuChar('\n').into()]))));
    assert_eq!(parse("''"), Ok(("", EuFn::from([EuChar('\'').into()]))));
}

#[test]
fn test_char_invalid() {
    assert!(parse(r#"'"#).is_err());
}

#[test]
fn test_fn() {
    assert_eq!(
        parse(r#"(1 "2" 3+ asdf)"#),
        Ok((
            "",
            EuFn::from([EuFn::from([
                EuI64(1).into(),
                EuStr::from("2").into(),
                EuI64(3).into(),
                EuWord::from("+").into(),
                EuWord::from("asdf").into()
            ])
            .into()])
        ))
    );
    assert_eq!(
        parse(r#"(1 "2" 3+ asdf"#),
        Ok((
            "",
            EuFn::from([EuFn::from([
                EuI64(1).into(),
                EuStr::from("2").into(),
                EuI64(3).into(),
                EuWord::from("+").into(),
                EuWord::from("asdf").into()
            ])
            .into()])
        ))
    );
    assert_eq!(
        parse(r#"((1 "2") 3+ (asdf))"#),
        Ok((
            "",
            EuFn::from([EuFn::from([
                EuFn::from([EuI64(1).into(), EuStr::from("2").into()]).into(),
                EuI64(3).into(),
                EuWord::from("+").into(),
                EuFn::from([EuWord::from("asdf").into()]).into()
            ])
            .into()])
        ))
    );
}

#[test]
fn test_all() {
    assert_eq!(
        parse(r#"1234"testing testing 123"#),
        Ok((
            "",
            EuFn::from([
                EuI64(1234).into(),
                EuStr::from("testing testing 123").into()
            ])
        ))
    );
    assert_eq!(
        parse(r#"asdf"testing testing 123"#),
        Ok((
            "",
            EuFn::from([
                EuWord::from("asdf").into(),
                EuStr::from("testing testing 123").into()
            ])
        ))
    );
    assert_eq!(
        parse("1234e5asdf"),
        Ok((
            "",
            EuFn::from([EuF64(1234e5).into(), EuWord::from("asdf").into()])
        ))
    );
    assert_eq!(
        parse("1234isizetest"),
        Ok((
            "",
            EuFn::from([EuIsize(1234).into(), EuWord::from("test").into()])
        ))
    );
    assert_eq!(
        parse("123ever"),
        Ok((
            "",
            EuFn::from([EuI64(123).into(), EuWord::from("ever").into()])
        ))
    );
    assert_eq!(
        parse("123e.4"),
        Ok((
            "",
            EuFn::from([EuI64(123).into(), EuWord::from("e.4").into()])
        ))
    );
    assert_eq!(
        parse("(1 2+)map"),
        Ok((
            "",
            EuFn::from([
                EuFn::from([EuI64(1).into(), EuI64(2).into(), EuWord::from("+").into()]).into(),
                EuWord::from("map").into()
            ])
        ))
    );
}

fn parse(input: &str) -> ModalResult<(&str, EuFn<'_>)> {
    euphrates.parse_peek(input)
}
