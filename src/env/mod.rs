mod state;
use std::mem;

pub use state::*;
use winnow::Parser as _;

use crate::{
    EvalResult,
    fns::CORE,
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

    pub fn new_state(&mut self) {
        self.xs.push(mem::take(&mut self.x));
    }

    pub fn pop_state(&mut self) -> Option<EuState<'_>> {
        self.xs.pop().map(|x| mem::replace(&mut self.x, x))
    }

    pub fn eval_str(&mut self, input: &str) -> EvalResult {
        euphrates
            .parse(input)
            .map_err(|e| Box::new(e.to_string()) as _)
            .and_then(|ts| self.eval_iter(ts))
    }

    pub fn eval_iter(&mut self, ts: impl IntoIterator<Item = EuType<'eu>>) -> EvalResult {
        for t in ts {
            self.eval_type(t)?;
            if self.x.done {
                break;
            }
        }
        Ok(())
    }

    fn eval_type(&mut self, t: EuType<'eu>) -> EvalResult {
        match t {
            EuType::Word(w) => {
                return if let Some((meta, f)) = CORE.get(&w) {
                    let meta = meta.name(&w);
                    self.x.check_nargs(meta)?;
                    f(self, meta)
                } else {
                    Err(Box::new(format!("unknown word `{w}`")))
                };
            }
            _ => self.x.stack.push(t),
        }
        Ok(())
    }

    fn find_var(&mut self, w: &str) -> Option<&EuType<'_>> {
        self.x.scope.get(w)
    }
}
