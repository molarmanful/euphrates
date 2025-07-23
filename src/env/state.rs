use std::collections::HashMap;

use anyhow::anyhow;
use derive_more::Display;
use ecow::EcoVec;
use hipstr::HipStr;

use crate::types::EuType;

#[derive(Debug, Clone, Display, Default)]
#[display("stack: {stack:?}\nscope: {scope:?}")]
pub struct EuState<'eu> {
    pub stack: EcoVec<EuType<'eu>>,
    pub scope: HashMap<HipStr<'eu>, EuType<'eu>>,
}

impl<'eu> EuState<'eu> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iflip(&self, i: usize) -> usize {
        self.stack.len() - i - 1
    }

    pub fn pop(&mut self) -> anyhow::Result<EuType<'eu>> {
        self.check_nargs(1).map(|_| self.stack.pop().unwrap())
    }

    pub fn split_off(&mut self, n: usize) -> anyhow::Result<EcoVec<EuType<'eu>>> {
        self.check_nargs(n)?;
        let (a, b) = self.stack.split_at(self.stack.len() - n);
        let b = b.into();
        self.stack = a.into();
        Ok(b)
    }

    pub fn check_nargs(&self, n: usize) -> anyhow::Result<()> {
        if self.stack.len() < n {
            Err(anyhow!(
                "actual stack len {} < {} expected",
                self.stack.len(),
                n,
            ))
        } else {
            Ok(())
        }
    }
}
