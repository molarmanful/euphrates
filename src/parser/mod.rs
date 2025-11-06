use ecow::EcoVec;
use hipstr::LocalHipStr;
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
        terminated,
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

use crate::types::{
    EuSyn,
    EuType,
};

pub fn euphrates<'eu>(input: &mut &str) -> ModalResult<EcoVec<EuSyn<'eu>>> {
    terminated(
        repeat(0.., preceded(ws, eu_syn)).fold(EcoVec::new, |mut ts, t| {
            ts.push(t);
            ts
        }),
        ws,
    )
    .parse_next(input)
}

fn eu_syn<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    dispatch!(peek(any);
        '`' => eu_str_raw,
        '"' => eu_str,
        '\'' => eu_char,
        '(' => eu_expr,
        ')' => fail,
        '[' => eu_vec,
        ']' => fail,
        '{' => eu_map,
        '}' => fail,
        '0'..='9' => eu_num,
        '$' => eu_var,
        _ => eu_word,
    )
    .parse_next(input)
}

fn eu_str_raw<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    delimited('`', take_while(0.., |c| c != '`'), opt('`'))
        .output_into()
        .map(EuType::Str)
        .map(EuSyn::Raw)
        .parse_next(input)
}

fn eu_str<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
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
    .map(EuType::str)
    .map(EuSyn::Raw)
    .parse_next(input)
}

fn eu_char<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    preceded(
        '\'',
        cut_err(eu_char_atom.verify_map(|x| x.map(EuType::Char)))
            .context(StrContext::Label("char")),
    )
    .map(EuSyn::Raw)
    .parse_next(input)
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

fn eu_expr<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    delimited('(', euphrates.map(EuType::expr).map(EuSyn::Raw), opt(')')).parse_next(input)
}

fn eu_vec<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    delimited('[', euphrates.map(EuSyn::Vec), opt(']')).parse_next(input)
}

fn eu_map<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    delimited('{', euphrates.map(EuSyn::Map), opt('}')).parse_next(input)
}

fn eu_num<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
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
    if dec.is_some() || exp.is_some() {
        eu_float_suffix(ns, input)
    } else {
        eu_int_suffix(ns, input)
    }
}

#[crabtime::function]
fn gen_fn_int_suffix() {
    let types = ["I32", "F32", "I64", "F64", "IBig"];
    let arms = types
        .map(|t| {
            let tl = format!("\"{}\"", t.to_lowercase());
            crabtime::quote! {
                {{tl}}
                    .try_map(|_| ns.parse().map(EuType::{{t}}))
                    .context(StrContext::Label({{tl}})),
            }
        })
        .join("");
    crabtime::output! {
        fn eu_int_suffix<'eu>(ns: &str, input: &mut &str) -> ModalResult<EuSyn<'eu>> {
            cut_err(alt((
                {{arms}}
                empty
                    .try_map(|_| ns.parse().map(EuType::IBig))
                    .context(StrContext::Label("int")),
            )))
            .map(EuSyn::Raw)
            .parse_next(input)
        }
    }
}

gen_fn_int_suffix!();

fn eu_float_suffix<'eu>(ns: &str, input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    cut_err(not(alt(("i32", "i64", "ibig"))))
        .context(StrContext::Label("float suffix"))
        .context(StrContext::Expected(StrContextValue::Description(
            "none of [`i32` `i64` `ibig`]",
        )))
        .parse_next(input)?;
    cut_err(alt((
        "f32"
            .try_map(|_| ns.parse().map(EuType::F32))
            .context(StrContext::Label("f32")),
        "f64"
            .try_map(|_| ns.parse().map(EuType::F64))
            .context(StrContext::Label("f64")),
        empty
            .try_map(|_| ns.parse().map(EuType::F64))
            .context(StrContext::Label("float")),
    )))
    .map(EuSyn::Raw)
    .parse_next(input)
}

fn eu_var<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    alt((
        preceded('$', eu_word_inner).output_into().map(EuSyn::Var),
        eu_word,
    ))
    .parse_next(input)
}

fn eu_word<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    eu_word_inner
        .map(EuType::Word)
        .map(EuSyn::Raw)
        .parse_next(input)
}

fn eu_word_inner<'eu>(input: &mut &str) -> ModalResult<LocalHipStr<'eu>> {
    take_while(1.., |c: char| {
        !matches!(c, '`' | '"' | '\'' | '(' | ')' | '[' | ']' | '{' | '}') && !c.is_whitespace()
    })
    .output_into()
    .parse_next(input)
}

fn ws<'i>(input: &mut &'i str) -> ModalResult<&'i str> {
    take_while(0.., char::is_whitespace).parse_next(input)
}

#[cfg(test)]
mod tests;
