mod core;
pub use core::*;

use crate::{
    EvalOption,
    env::EuEnv,
};

pub type EuDef<'s, 'e> = (EuFnMeta<'s>, EuFn<'e>);

pub type EuFn<'e> = fn(&mut EuEnv, EuFnMeta) -> EvalOption<'e>;

#[derive(Debug, Clone, Copy)]
pub struct EuFnMeta<'s> {
    pub name: &'s str,
    pub nargs: usize,
}

pub const META: EuFnMeta = EuFnMeta::new();

impl<'s> EuFnMeta<'s> {
    pub const fn new() -> Self {
        Self { name: "", nargs: 0 }
    }

    pub const fn name(mut self, name: &'s str) -> Self {
        self.name = name;
        self
    }

    pub const fn nargs(mut self, nargs: usize) -> Self {
        self.nargs = nargs;
        self
    }
}
