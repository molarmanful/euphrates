use std::collections::HashMap;

use derive_more::Display;
use ecow::EcoVec;

use crate::{
    EvalError,
    fns::EuFnMeta,
    types::EuType,
};

#[derive(Debug, Clone, Display, Default)]
#[display("stack: {stack:?}\nscope: {scope:?}")]
pub struct EuState<'eu> {
    pub stack: EcoVec<EuType<'eu>>,
    pub scope: HashMap<&'eu str, EuType<'eu>>,
}

impl EuState<'_> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iflip(&self, i: usize) -> usize {
        self.stack.len() - i - 1
    }

    pub fn split_off(&mut self, n: usize) -> Self {
        let (a, b) = self.stack.split_at(self.stack.len() - n);
        let new = Self {
            stack: b.into(),
            ..Self::new()
        };
        self.stack = a.into();
        new
    }

    pub fn err_nargs(&self, meta: EuFnMeta) -> EvalError {
        format!(
            "(stack len) {} < {} ({})",
            self.stack.len(),
            meta.nargs,
            meta.name
        )
        .into()
    }
}
