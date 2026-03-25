mod base;
mod bind;

use ecow::EcoVec;
use winnow::{
    Parser,
    error::{
        ContextError,
        ParseError,
    },
};

use crate::{
    parser::euphrates,
    types::EuSyn,
};

fn parse(input: &str) -> Result<EcoVec<EuSyn<'_>>, ParseError<&str, ContextError>> {
    euphrates.parse(input)
}

fn is_err(input: &str) -> bool {
    euphrates.parse(input).is_err()
}
