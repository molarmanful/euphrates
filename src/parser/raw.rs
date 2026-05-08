use winnow::{
    combinator::{
        cut_err,
        delimited,
        dispatch,
        fail,
        opt,
        peek,
    },
    error::StrContext,
    prelude::*,
    token::any,
};

use crate::{
    parser::{
        char_inner,
        euphrates,
        float_suffix,
        int_suffix,
        num_inner,
        str_inner,
        str_raw_inner,
        word_inner,
    },
    types::{
        EuSyn,
        EuType,
    },
};

pub(super) fn raw<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    raw_inner.map(EuSyn::Raw).parse_next(input)
}

pub(super) fn raw_inner<'eu>(input: &mut &str) -> ModalResult<EuType<'eu>> {
    dispatch!(peek(any);
        '(' => expr,
        ')' => fail,
        '`' => str_raw,
        '"' => str,
        '\'' => char,
        '0'..='9' => num,
        _ => word,
    )
    .parse_next(input)
}

fn expr<'eu>(input: &mut &str) -> ModalResult<EuType<'eu>> {
    delimited('(', euphrates, opt(')'))
        .map(EuType::expr)
        .context(StrContext::Label("expr"))
        .parse_next(input)
}

fn str_raw<'eu>(input: &mut &str) -> ModalResult<EuType<'eu>> {
    str_raw_inner.map(EuType::Str).parse_next(input)
}

fn str<'eu>(input: &mut &str) -> ModalResult<EuType<'eu>> {
    str_inner.map(EuType::Str).parse_next(input)
}

fn char<'eu>(input: &mut &str) -> ModalResult<EuType<'eu>> {
    cut_err(
        char_inner
            .verify_map(|x| x.map(EuType::Char))
            .context(StrContext::Label("char")),
    )
    .parse_next(input)
}

fn num<'eu>(input: &mut &str) -> ModalResult<EuType<'eu>> {
    let (is_float, ns) = num_inner.parse_next(input)?;
    if is_float {
        float_suffix(ns, EuType::F64).parse_next(input)
    } else {
        int_suffix(ns, EuType::I32, EuType::I64, EuType::IBig, EuType::F64).parse_next(input)
    }
}

fn word<'eu>(input: &mut &str) -> ModalResult<EuType<'eu>> {
    word_inner
        .map(EuType::Word)
        .context(StrContext::Label("word"))
        .parse_next(input)
}
