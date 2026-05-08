use ecow::EcoVec;
use winnow::{
    combinator::{
        cut_err,
        delimited,
        dispatch,
        opt,
        peek,
        preceded,
        repeat,
        separated_pair,
        terminated,
    },
    error::StrContext,
    prelude::*,
    token::any,
};

use crate::{
    parser::{
        char_inner,
        float_suffix,
        int_suffix,
        num_inner,
        str_inner,
        str_raw_inner,
        word_inner,
        ws,
    },
    types::{
        EuBind,
        EuSyn,
    },
};

pub(super) fn bind<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    delimited('[', bind_inner, opt(']'))
        .map(EuSyn::Bind)
        .context(StrContext::Label("bind"))
        .parse_next(input)
}

fn bind_inner<'eu>(input: &mut &str) -> ModalResult<EcoVec<EuBind<'eu>>> {
    terminated(
        repeat(0.., preceded(ws, syn)).fold(EcoVec::new, |mut ts, t| {
            ts.push(t);
            ts
        }),
        ws,
    )
    .parse_next(input)
}

fn syn<'eu>(input: &mut &str) -> ModalResult<EuBind<'eu>> {
    (
        dispatch!(peek(any);
            '`' => str_raw,
            '"' => str,
            '\'' => char,
            '0'..='9' => num,
            '(' => union,
            '[' => vecz,
            '{' => map,
            '$' => tag,
            _ => word,
        ),
        opt(bind_),
    )
        .map(|(b, o)| {
            if let Some(b1) = o {
                EuBind::bind(b, b1)
            } else {
                b
            }
        })
        .parse_next(input)
}

fn str_raw<'eu>(input: &mut &str) -> ModalResult<EuBind<'eu>> {
    str_raw_inner.map(EuBind::Str).parse_next(input)
}

fn str<'eu>(input: &mut &str) -> ModalResult<EuBind<'eu>> {
    str_inner.map(EuBind::Str).parse_next(input)
}

fn char<'eu>(input: &mut &str) -> ModalResult<EuBind<'eu>> {
    cut_err(
        char_inner
            .verify_map(|x| x.map(EuBind::Char))
            .context(StrContext::Label("char")),
    )
    .parse_next(input)
}

fn union<'eu>(input: &mut &str) -> ModalResult<EuBind<'eu>> {
    delimited('(', bind_inner, opt(')'))
        .map(EuBind::Union)
        .context(StrContext::Label("union"))
        .parse_next(input)
}

fn vecz<'eu>(input: &mut &str) -> ModalResult<EuBind<'eu>> {
    delimited('[', bind_inner, opt(']'))
        .map(EuBind::Vecz)
        .parse_next(input)
}

fn map<'eu>(input: &mut &str) -> ModalResult<EuBind<'eu>> {
    delimited('{', bind_inner, opt('}'))
        .map(EuBind::Map)
        .parse_next(input)
}

fn num<'eu>(input: &mut &str) -> ModalResult<EuBind<'eu>> {
    let (is_float, ns) = num_inner.parse_next(input)?;
    if is_float {
        float_suffix(ns, EuBind::F64).parse_next(input)
    } else {
        int_suffix(ns, EuBind::I32, EuBind::I64, EuBind::IBig, EuBind::F64).parse_next(input)
    }
}

fn tag<'eu>(input: &mut &str) -> ModalResult<EuBind<'eu>> {
    cut_err(separated_pair(
        preceded('$', word_inner),
        ws,
        delimited('(', bind_inner, opt(')')),
    ))
    .map(|(w, bs)| EuBind::Tag(w, bs))
    .context(StrContext::Label("tag"))
    .parse_next(input)
}

fn bind_<'eu>(input: &mut &str) -> ModalResult<EuBind<'eu>> {
    preceded((ws, '\\', ws), syn)
        .context(StrContext::Label("bind"))
        .parse_next(input)
}

fn word<'eu>(input: &mut &str) -> ModalResult<EuBind<'eu>> {
    word_inner
        .map(EuBind::Word)
        .context(StrContext::Label("word"))
        .parse_next(input)
}
