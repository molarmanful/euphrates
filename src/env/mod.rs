mod state;
use std::mem;

pub use state::*;
use winnow::Parser as _;

use crate::{
    EvalOption,
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

    pub fn pop_state(&mut self) -> bool {
        if let Some(x) = self.xs.pop() {
            self.x = x;
            true
        } else {
            false
        }
    }

    pub fn eval_str(&mut self, input: &str) -> EvalOption<'_> {
        match euphrates.parse(input) {
            Ok(ts) => self.eval_iter(ts),
            Err(e) => Some(e.to_string().into()),
        }
    }

    fn eval_iter(&mut self, ts: impl IntoIterator<Item = EuType<'eu>>) -> EvalOption<'_> {
        for t in ts {
            match t {
                EuType::Word(w) => {
                    if let Some((meta, f)) = CORE.get(&w) {
                        let meta = meta.name(&w);
                        if self.x.stack.len() < meta.nargs {
                            return Some(self.x.err_nargs(meta));
                        }
                        if let e @ Some(_) = f(self, meta) {
                            return e;
                        }
                    } else {
                        return Some(format!("unknown word {w}").into());
                    }
                }
                _ => self.x.stack.push(t),
            }
        }
        None
    }
}
