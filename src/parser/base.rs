use ecow::EcoVec;
use winnow::{
    combinator::{
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
        raw::raw,
        word_inner,
        ws,
    },
    types::EuSyn,
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
        '[' => vec,
        ']' => fail,
        '{' => map,
        '}' => fail,
        '$' => var,
        '\\' => preceded(
            '\\',
            dispatch!(peek(any);
                '[' => bind,
                _ => r#move,
            ),
        ),
        '.' => get,
        _ => raw,
    )
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

fn var<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    preceded('$', word_inner)
        .map(EuSyn::Var)
        .context(StrContext::Label("var"))
        .parse_next(input)
}

fn r#move<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    word_inner
        .map(EuSyn::Move)
        .context(StrContext::Label("move"))
        .parse_next(input)
}

fn get<'eu>(input: &mut &str) -> ModalResult<EuSyn<'eu>> {
    preceded(('.', ws), word_inner)
        .map(EuSyn::Get)
        .parse_next(input)
}
