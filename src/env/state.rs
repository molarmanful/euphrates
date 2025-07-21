use std::collections::HashMap;

use derive_more::Display;
use ecow::EcoVec;

use crate::{
    EvalError,
    EvalResult,
    fns::EuFnMeta,
    types::EuType,
};

#[derive(Debug, Clone, Display, Default)]
#[display("stack: {stack:?}\nscope: {scope:?}")]
pub struct EuState<'eu> {
    pub mode: EuStateMode,
    pub stack: EcoVec<EuType<'eu>>,
    pub scope: HashMap<&'eu str, EuType<'eu>>,
    pub done: bool,
}

#[derive(Debug, Clone, Default)]
pub enum EuStateMode {
    #[default]
    X,
    FN,
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

    pub fn check_nargs(&self, meta: EuFnMeta) -> EvalResult {
        if self.stack.len() < meta.nargs {
            Err(self.err_nargs(meta))
        } else {
            Ok(())
        }
    }

    fn err_nargs(&self, meta: EuFnMeta) -> EvalError {
        Box::new(format!(
            "(stack len) {} < {} ({})",
            self.stack.len(),
            meta.nargs,
            meta.name
        ))
    }
}
