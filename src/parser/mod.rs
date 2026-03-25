mod base;
mod bind;

pub use base::euphrates;
use dashu_int::IBig;
use hipstr::LocalHipStr;
use ordered_float::OrderedFloat;
use winnow::{
    ascii::digit1,
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
    prelude::*,
    token::{
        any,
        one_of,
        take_while,
    },
};

fn eu_str_raw_inner<'eu>(input: &mut &str) -> ModalResult<LocalHipStr<'eu>> {
    delimited('`', take_while(0.., |c| c != '`'), opt('`'))
        .output_into()
        .parse_next(input)
}

fn eu_str_inner<'eu>(input: &mut &str) -> ModalResult<LocalHipStr<'eu>> {
    delimited(
        '"',
        repeat(0.., dispatch!(peek(any); '"' => fail, _ => eu_char_atom)).fold(
            LocalHipStr::new,
            |mut s, co| {
                if let Some(c) = co {
                    s.push(c);
                }
                s
            },
        ),
        opt('"'),
    )
    .parse_next(input)
}

fn eu_char_inner(input: &mut &str) -> ModalResult<Option<char>> {
    preceded('\'', eu_char_atom).parse_next(input)
}

fn eu_char_atom(input: &mut &str) -> ModalResult<Option<char>> {
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

fn eu_char_hex(input: &mut &str) -> ModalResult<char> {
    cut_err(
        take_while(2, |c: char| c.is_ascii_hexdigit())
            .try_map(|hex| u8::from_str_radix(hex, 16))
            .output_into(),
    )
    .context(StrContext::Label("hex pair escape"))
    .context(StrContext::Expected(StrContextValue::Description(
        "`\\xHH` where `H` is a hexadecimal digit",
    )))
    .parse_next(input)
}

fn eu_char_uni(input: &mut &str) -> ModalResult<char> {
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

fn int_suffix<T>(
    ns: &str,
    i32: impl FnOnce(i32) -> T + Copy,
    i64: impl FnOnce(i64) -> T + Copy,
    ibig: impl FnOnce(IBig) -> T + Copy,
    f64: impl FnOnce(OrderedFloat<f64>) -> T + Copy,
) -> impl FnMut(&mut &str) -> ModalResult<T> {
    move |input| {
        cut_err(alt((
            "i32"
                .try_map(|_| ns.parse().map(i32))
                .context(StrContext::Label("i32")),
            "i64"
                .try_map(|_| ns.parse().map(i64))
                .context(StrContext::Label("i64")),
            "ibig"
                .try_map(|_| ns.parse().map(ibig))
                .context(StrContext::Label("ibig")),
            "f64"
                .try_map(|_| ns.parse().map(f64))
                .context(StrContext::Label("f64")),
            empty
                .try_map(|()| ns.parse().map(ibig))
                .context(StrContext::Label("int")),
        )))
        .parse_next(input)
    }
}

fn float_suffix<T>(
    ns: &str,
    f64: impl FnOnce(OrderedFloat<f64>) -> T + Copy,
) -> impl FnMut(&mut &str) -> ModalResult<T> {
    move |input| {
        not_int_suffix(input)?;
        cut_err(alt((
            "f64"
                .try_map(|_| ns.parse().map(f64))
                .context(StrContext::Label("f64")),
            empty
                .try_map(|()| ns.parse().map(f64))
                .context(StrContext::Label("float")),
        )))
        .parse_next(input)
    }
}

fn not_int_suffix(input: &mut &str) -> ModalResult<()> {
    cut_err(not(alt(("i32", "i64", "ibig"))))
        .context(StrContext::Label("float suffix"))
        .context(StrContext::Expected(StrContextValue::Description(
            "`f64` or no suffix",
        )))
        .parse_next(input)
}

fn eu_num_inner<'i>(input: &mut &'i str) -> ModalResult<(bool, &'i str)> {
    let ((_, dec, exp), ns) = (
        digit1,
        opt(preceded('.', digit1)),
        opt((one_of(('e', 'E')), opt(one_of(('+', '-'))), digit1)),
    )
        .verify_map(|res @ (pre, dec, _): (&str, _, _)| {
            (!pre.is_empty() || dec.is_some_and(|s: &str| !s.is_empty())).then_some(res)
        })
        .with_taken()
        .parse_next(input)?;
    Ok((dec.is_some() || exp.is_some(), ns))
}

fn eu_word_inner<'eu>(input: &mut &str) -> ModalResult<LocalHipStr<'eu>> {
    (
        take_while(1, |c: char| {
            !c.is_dec_digit() && !matches!(c, '$' | '\\') && is_word_char(c)
        }),
        take_while(0.., is_word_char),
    )
        .map(|(a, b)| LocalHipStr::concat([a, b]))
        .parse_next(input)
}

fn is_word_char(c: char) -> bool {
    !matches!(
        c,
        '`' | '"' | '\'' | '(' | ')' | '[' | ']' | '{' | '}' | '.' | '\\'
    ) && !c.is_whitespace()
}

fn ws<'i>(input: &mut &'i str) -> ModalResult<&'i str> {
    take_while(0.., char::is_whitespace).parse_next(input)
}

#[cfg(test)]
mod tests;
