mod state;
use std::mem;

use anyhow::{
    Context,
    anyhow,
};
pub use state::*;
use winnow::Parser as _;

use crate::{
    fns::{
        CONSTS,
        CORE,
    },
    parser::euphrates,
    types::EuType,
};

#[derive(Debug, Default)]
pub struct EuEnv<'eu> {
    pub x: EuState<'eu>,
    pub xs: Vec<EuState<'eu>>,
}

impl<'eu> EuEnv<'eu> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn eval_str(&mut self, input: &str) -> anyhow::Result<()> {
        euphrates
            .parse(input)
            .map_err(|e| anyhow!(e.to_string()))
            .and_then(|ts| self.eval_iter(ts))
    }

    pub fn eval_iter_scoped(
        &mut self,
        ts: impl IntoIterator<Item = EuType<'eu>>,
    ) -> anyhow::Result<()> {
        self.xs.push(mem::take(&mut self.x));
        self.eval_iter(ts)?;
        self.xs.pop().map(|x| mem::replace(&mut self.x, x));
        Ok(())
    }

    pub fn eval_iter(&mut self, ts: impl IntoIterator<Item = EuType<'eu>>) -> anyhow::Result<()> {
        for t in ts {
            self.eval_type(t)?;
        }
        Ok(())
    }

    fn eval_type(&mut self, t: EuType<'eu>) -> anyhow::Result<()> {
        match t {
            EuType::Word(w) => {
                return if let Some(v) = CONSTS.get(&w) {
                    self.x.stack.push(v.clone());
                    Ok(())
                } else if let Some(f) = CORE.get(&w) {
                    f(self).with_context(|| format!("`{w}` failed"))
                } else {
                    Err(anyhow!("unknown word `{w}`"))
                };
            }
            _ => self.x.stack.push(t),
        }
        Ok(())
    }

    pub fn parent(&mut self) -> anyhow::Result<&mut EuState<'eu>> {
        self.xs.last_mut().context("invalid call from root scope")
    }

    fn get_var(&mut self, w: &str) -> Option<&EuType<'_>> {
        self.x.scope.get(w)
    }
}
