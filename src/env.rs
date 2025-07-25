use std::{
    borrow::Cow,
    collections::HashMap,
};

use anyhow::{
    Context,
    anyhow,
};
use derive_more::Display;
use ecow::EcoVec;
use hipstr::HipStr;
use winnow::Parser as _;

use crate::{
    fns::{
        CONSTS,
        CORE,
    },
    parser::euphrates,
    types::EuType,
};

#[derive(Debug, Display, Clone, Default)]
#[display("stack: {stack:?}\nscope: {scope:?}")]
pub struct EuEnv<'eu> {
    pub stack: EcoVec<EuType<'eu>>,
    pub scope: Cow<'eu, HashMap<HipStr<'eu>, EuType<'eu>>>,
}

impl<'eu> EuEnv<'eu> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn eval_str(&mut self, input: &str) -> anyhow::Result<()> {
        euphrates
            .parse(input)
            .map_err(|e| anyhow!(e.to_string()))
            .and_then(|ts| self.eval_iter(ts))
    }

    pub fn eval_iter(&mut self, ts: impl IntoIterator<Item = EuType<'eu>>) -> anyhow::Result<()> {
        for t in ts {
            println!("{t:?}\n>>>");
            self.eval_type(t)?;
            println!("{self}\n<<<");
        }
        Ok(())
    }

    fn eval_type(&mut self, t: EuType<'eu>) -> anyhow::Result<()> {
        match t {
            EuType::Word(w) => self.eval_word(&w),
            _ => {
                self.stack.push(t);
                Ok(())
            }
        }
    }

    fn eval_word(&mut self, w: &str) -> anyhow::Result<()> {
        if let Some(v) = self.scope.get(w) {
            if let EuType::Expr(ts) = v {
                self.eval_iter(ts.clone())
            } else {
                self.stack.push(v.clone());
                Ok(())
            }
        } else if let Some(v) = CONSTS.get(&w) {
            self.stack.push(v.clone());
            Ok(())
        } else if let Some(f) = CORE.get(&w) {
            f(self).with_context(|| format!("`{w}` failed"))
        } else {
            Err(anyhow!("unknown word `{w}`"))
        }
    }

    pub fn bind_args(&mut self, ts: EcoVec<EuType<'eu>>) -> anyhow::Result<()> {
        for t in ts.into_iter().rev() {
            let v = self.stack.pop().context("insufficient args passed")?;
            match t {
                EuType::Word(w) => self.scope.to_mut().insert(w, v),
                _ => todo!(),
            };
        }
        Ok(())
    }

    #[inline]
    pub fn pop(&mut self) -> anyhow::Result<EuType<'eu>> {
        self.check_nargs(1).map(|_| self.stack.pop().unwrap())
    }

    #[inline]
    pub fn iflip(&self, i: usize) -> usize {
        self.stack.len() - i - 1
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
