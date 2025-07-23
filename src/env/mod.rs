mod state;
use std::{
    iter,
    mem,
};

use anyhow::{
    Context,
    anyhow,
};
use ecow::EcoVec;
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
            EuType::Word(w) => self.eval_word(&w),
            _ => {
                self.x.stack.push(t);
                Ok(())
            }
        }
    }

    fn eval_word(&mut self, w: &str) -> anyhow::Result<()> {
        if let Some(v) = self.get_var(&w) {
            if let EuType::Expr(ts) = v {
                self.eval_iter_scoped(ts.clone())
            } else {
                self.x.stack.push(v.clone());
                Ok(())
            }
        } else if let Some(v) = CONSTS.get(&w) {
            self.x.stack.push(v.clone());
            Ok(())
        } else if let Some(f) = CORE.get(&w) {
            f(self).with_context(|| format!("`{w}` failed"))
        } else {
            Err(anyhow!("unknown word `{w}`"))
        }
    }

    pub fn pull_args(&mut self, ts: EcoVec<EuType<'eu>>) -> anyhow::Result<()> {
        let prev = ctx_root_scope(self.xs.last_mut())?;
        for t in ts.into_iter().rev() {
            let v = prev.stack.pop().context("insufficient args passed")?;
            match t {
                EuType::Word(w) => self.x.scope.insert(w, v),
                _ => todo!(),
            };
        }
        Ok(())
    }

    pub fn parent(&mut self) -> anyhow::Result<&mut EuState<'eu>> {
        ctx_root_scope(self.xs.last_mut())
    }

    fn get_var(&self, w: &str) -> Option<&EuType<'eu>> {
        iter::once(&self.x)
            .chain(self.xs.iter().rev())
            .find_map(|st| st.scope.get(w))
    }
}

fn ctx_root_scope<T, E, C: anyhow::Context<T, E>>(c: C) -> anyhow::Result<T> {
    c.context("invalid call from root scope")
}
