use ecow::EcoVec;
use winnow::{
    combinator::{
        cut_err,
        delimited,
        dispatch,
        fail,
        opt,
        peek,
        preceded,
        repeat,
        terminated,
    },
    error::StrContext,
    prelude::*,
    token::any,
};

use crate::{
    parser::{
        bind::bind,
        eu_char_inner,
        eu_num_inner,
        eu_str_inner,
        eu_str_raw_inner,
        eu_word_inner,
        float_suffix,
        int_suffix,
        ws,
    },
    types::{
        EuSyn,
        EuType,
    },
};

pub fn euphrates<'eu>(input: &mut &str) -> ModalResult<EcoVec<EuSyn<'eu>>> {
    terminated(
        repeat(0.., preceded(ws, syn)).fold(EcoVec::new, |mut ts, t| {
            ts.push(t);
            ts
        }),
        ws,
    )
    .parse_next(input)
}

fn syn<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    dispatch!(peek(any);
        '`' => str_raw,
        '"' => str,
        '\'' => char,
        '(' => expr,
        ')' => fail,
        '[' => vec,
        ']' => fail,
        '{' => map,
        '}' => fail,
        '0'..='9' => eu_num,
        '$' => var,
        '\\' => preceded(
            '\\',
            dispatch!(peek(any);
                '[' => bind,
                _ => r#move,
            ),
        ),
        _ => word,
    )
    .parse_next(input)
}

fn str_raw<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    eu_str_raw_inner
        .map(EuType::Str)
        .map(EuSyn::Raw)
        .parse_next(input)
}

fn str<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    eu_str_inner
        .map(EuType::Str)
        .map(EuSyn::Raw)
        .parse_next(input)
}

fn char<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    cut_err(
        eu_char_inner
            .verify_map(|x| x.map(EuType::Char).map(EuSyn::Raw))
            .context(StrContext::Label("char")),
    )
    .parse_next(input)
}

fn expr<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    delimited('(', euphrates, opt(')'))
        .map(EuType::expr)
        .map(EuSyn::Raw)
        .context(StrContext::Label("expr"))
        .parse_next(input)
}

fn vec<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    delimited('[', euphrates, opt(']'))
        .map(EuSyn::Vec)
        .parse_next(input)
}

fn map<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    delimited('{', euphrates, opt('}'))
        .map(EuSyn::Map)
        .parse_next(input)
}

fn eu_num<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    let (is_float, ns) = eu_num_inner.parse_next(input)?;
    if is_float {
        float_suffix(ns, EuType::F64)
            .map(EuSyn::Raw)
            .parse_next(input)
    } else {
        int_suffix(ns, EuType::I32, EuType::I64, EuType::IBig, EuType::F64)
            .map(EuSyn::Raw)
            .parse_next(input)
    }
}

fn var<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    preceded('$', eu_word_inner)
        .map(EuSyn::Var)
        .context(StrContext::Label("var"))
        .parse_next(input)
}

fn r#move<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    eu_word_inner
        .map(EuSyn::Move)
        .context(StrContext::Label("move"))
        .parse_next(input)
}

fn word<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    eu_word_inner
        .map(EuType::Word)
        .map(EuSyn::Raw)
        .context(StrContext::Label("word"))
        .parse_next(input)
}
