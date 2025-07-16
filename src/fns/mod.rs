mod core;
pub use core::*;

use crate::{
    EvalOption,
    state::EuState,
};

pub type EuDef<'st> = (EuFnMeta<'st>, EuFn);

pub type EuFn = for<'st> fn(&mut EuState<'st>, &EuFnMeta) -> EvalOption<'st>;

pub struct EuFnMeta<'st> {
    pub name: &'st str,
    pub nargs: usize,
}

impl EuFnMeta<'_> {
    const fn new() -> Self {
        Self { name: "", nargs: 0 }
    }

    const fn nargs(n: usize) -> Self {
        Self { name: "", nargs: n }
    }
}
