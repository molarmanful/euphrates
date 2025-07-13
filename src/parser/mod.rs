use hipstr::string::HipStr;
use winnow::{
    ModalResult,
    Parser,
    ascii::{
        digit0,
        digit1,
    },
    combinator::{
        alt,
        cut_err,
        delimited,
        dispatch,
        empty,
        fail,
        not,
        opt,
        peek,
        preceded,
        repeat,
    },
    error::{
        StrContext,
        StrContextValue,
    },
    token::{
        any,
        one_of,
        take_while,
    },
};

use crate::types::{
    EuChar,
    EuF32,
    EuF64,
    EuFn,
    EuI32,
    EuI64,
    EuIsize,
    EuStr,
    EuType,
    EuU32,
    EuU64,
    EuUsize,
    EuWord,
};

pub fn euphrates<'i>(input: &mut &'i str) -> ModalResult<EuFn<'i>> {
    delimited(ws, repeat(0.., preceded(ws, eu_type)).map(EuFn), ws).parse_next(input)
}

fn eu_type<'i>(input: &mut &'i str) -> ModalResult<EuType<'i>> {
    dispatch!(peek(any);
        '`' => eu_str_raw,
        '"' => eu_str,
        '\'' => eu_char,
        '(' => eu_fn,
        ')' => fail,
        '.' => alt((eu_num, eu_word)),
        '0'..='9' => eu_num,
        _ => eu_word,
    )
    .parse_next(input)
}

fn eu_str<'i>(input: &mut &'i str) -> ModalResult<EuType<'i>> {
    delimited(
        '"',
        repeat(0.., dispatch!(peek(any); '"' => fail, _ => eu_char_atom)).fold(
            HipStr::new,
            |mut s, co| {
                if let Some(c) = co {
                    s.push(c);
                }
                s
            },
        ),
        opt('"'),
    )
    .map(|s| EuStr(s).into())
    .parse_next(input)
}

fn eu_str_raw<'i>(input: &mut &'i str) -> ModalResult<EuType<'i>> {
    delimited('`', take_while(0.., |c| c != '`'), opt('`'))
        .map(|s| EuStr::from(s).into())
        .parse_next(input)
}

fn eu_char<'i>(input: &mut &'i str) -> ModalResult<EuType<'i>> {
    preceded(
        '\'',
        cut_err(eu_char_atom.verify_map(|x| x.map(|c| EuChar(c).into())))
            .context(StrContext::Label("char")),
    )
    .parse_next(input)
}

fn eu_char_atom<'i>(input: &mut &'i str) -> ModalResult<Option<char>> {
    dispatch!(any;
        '\\' => dispatch!(cut_err(any).context(StrContext::Label("escape"));
            '\n' => empty.value(None),
            '0' => empty.value(Some('\0')),
            'n' => empty.value(Some('\n')),
            'r' => empty.value(Some('\r')),
            't' => empty.value(Some('\t')),
            'x' => eu_char_hex.map(Some),
            'u' => eu_char_uni.map(Some),
            c => empty.value(Some(c)),
        ),
        c => empty.value(Some(c)),
    )
    .parse_next(input)
}

fn eu_char_hex<'i>(input: &mut &'i str) -> ModalResult<char> {
    cut_err(
        take_while(2, |c: char| c.is_ascii_hexdigit())
            .try_map(|hex| u8::from_str_radix(hex, 16))
            .map(u8::into),
    )
    .context(StrContext::Label("hex pair escape"))
    .context(StrContext::Expected(StrContextValue::Description(
        "`\\xHH` where `H` is a hexadecimal digit",
    )))
    .parse_next(input)
}

fn eu_char_uni<'i>(input: &mut &'i str) -> ModalResult<char> {
    cut_err(
        delimited(
            '{',
            take_while(1..=6, |c: char| c.is_ascii_hexdigit()),
            opt('}'),
        )
        .context(StrContext::Label("unicode escape"))
        .context(StrContext::Expected(
            "`\\u{H...}` where `H...` is 1-6 hexadecimal digits".into(),
        ))
        .try_map(|hex| u32::from_str_radix(hex, 16))
        .verify_map(char::from_u32),
    )
    .context(StrContext::Label("unicode escape"))
    .context(StrContext::Expected(
        winnow::error::StrContextValue::Description("valid codepoint"),
    ))
    .parse_next(input)
}

fn eu_fn<'i>(input: &mut &'i str) -> ModalResult<EuType<'i>> {
    delimited('(', euphrates.map(EuType::from), opt(')')).parse_next(input)
}

fn eu_num<'i>(input: &mut &'i str) -> ModalResult<EuType<'i>> {
    let ((_, dec, exp), ns) = (
        digit0,
        opt(preceded('.', digit0)),
        opt((one_of(('e', 'E')), opt(one_of(('+', '-'))), digit1)),
    )
        .verify_map(|res @ (pre, dec, _): (&str, _, _)| {
            (pre != "" || dec.is_some_and(|s| s != "")).then_some(res)
        })
        .with_taken()
        .parse_next(input)?;
    if dec.is_some() || exp.is_some() {
        eu_float_suffix(ns, input)
    } else {
        eu_int_suffix(ns, input)
    }
}

fn eu_int_suffix<'eu>(ns: &str, input: &mut &str) -> ModalResult<EuType<'eu>> {
    cut_err(alt((
        "isize"
            .try_map(|_| ns.parse().map(|n| EuIsize(n).into()))
            .context(StrContext::Label("ISize")),
        "usize"
            .try_map(|_| ns.parse().map(|n| EuUsize(n).into()))
            .context(StrContext::Label("USize")),
        "i32"
            .try_map(|_| ns.parse().map(|n| EuI32(n).into()))
            .context(StrContext::Label("I32")),
        "u32"
            .try_map(|_| ns.parse().map(|n| EuU32(n).into()))
            .context(StrContext::Label("U32")),
        "f32"
            .try_map(|_| ns.parse().map(|n| EuF32(n).into()))
            .context(StrContext::Label("F32")),
        "i64"
            .try_map(|_| ns.parse().map(|n| EuI64(n).into()))
            .context(StrContext::Label("I64")),
        "u64"
            .try_map(|_| ns.parse().map(|n| EuU64(n).into()))
            .context(StrContext::Label("U64")),
        "f64"
            .try_map(|_| ns.parse().map(|n| EuF64(n).into()))
            .context(StrContext::Label("F64")),
        empty.try_map(|_| ns.parse().map(|n| EuI64(n).into())),
    )))
    .parse_next(input)
}

fn eu_float_suffix<'eu>(ns: &str, input: &mut &str) -> ModalResult<EuType<'eu>> {
    cut_err(not(alt(("isize", "usize", "i32", "u32", "i64", "u64"))))
        .context(StrContext::Label("float suffix"))
        .context(StrContext::Expected(StrContextValue::Description(
            "none of [`isize` `usize` `i32` `u32` `i64` `u64`]",
        )))
        .parse_next(input)?;
    cut_err(alt((
        "f32"
            .try_map(|_| ns.parse().map(|n| EuF32(n).into()))
            .context(StrContext::Label("F32")),
        "f64"
            .try_map(|_| ns.parse().map(|n| EuF64(n).into()))
            .context(StrContext::Label("F64")),
        empty.try_map(|_| ns.parse().map(|n| EuF64(n).into())),
    )))
    .parse_next(input)
}

fn eu_word<'i>(input: &mut &'i str) -> ModalResult<EuType<'i>> {
    take_while(0.., |c: char| {
        !matches!(c, '`' | '"' | '\'' | '(' | ')') && !c.is_whitespace()
    })
    .map(|s| EuWord(HipStr::borrowed(s)).into())
    .parse_next(input)
}

fn ws<'i>(input: &mut &'i str) -> ModalResult<&'i str> {
    take_while(0.., char::is_whitespace).parse_next(input)
}

#[cfg(test)]
mod tests;
