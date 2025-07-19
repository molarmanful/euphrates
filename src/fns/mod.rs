mod core;
pub use core::*;

use crate::{
    EvalOption,
    state::EuState,
};

pub type EuDef<'st> = (EuFnMeta<'st>, EuFn);

pub type EuFn = for<'st> fn(&mut EuState<'st>, &EuFnMeta) -> EvalOption<'st>;

#[derive(Clone, Copy)]
pub struct EuFnMeta<'st> {
    pub name: &'st str,
    pub nargs: usize,
}

pub const META: EuFnMeta = EuFnMeta::new();

impl<'st> EuFnMeta<'st> {
    pub const fn new() -> Self {
        Self { name: "", nargs: 0 }
    }

    pub const fn name(mut self, name: &'st str) -> Self {
        self.name = name;
        self
    }

    pub const fn nargs(mut self, nargs: usize) -> Self {
        self.nargs = nargs;
        self
    }
}
