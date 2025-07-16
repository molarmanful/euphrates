use std::collections::HashMap;

use derive_more::Display;
use winnow::Parser as _;

use crate::{
    EvalOption,
    EvalResult,
    fns::{
        CORE,
        EuFnMeta,
    },
    parser::euphrates,
    types::{
        EuType,
        EuVec,
    },
};

#[derive(Debug, Display, Clone)]
#[display("stack: {stack:?}\nscope: {scope:?}")]
pub struct EuState<'st> {
    pub stack: EuVec<'st>,
    pub scope: HashMap<&'st str, EuType<'st>>,
}

impl<'st> EuState<'st> {
    pub fn new() -> Self {
        Self {
            stack: EuVec::from([]),
            scope: HashMap::new(),
        }
    }

    pub fn run(s: &'st str) -> EvalResult<'st> {
        let mut st = Self::new();
        st.eval_str(s).map_or_else(|| Ok(st), Err)
    }

    fn eval_str(&mut self, s: &'st str) -> EvalOption<'st> {
        match euphrates.parse(s) {
            Ok(xs) => self.eval_vec(xs),
            Err(e) => Some(Box::new(e)),
        }
    }

    fn eval_vec(&mut self, f: EuVec<'st>) -> EvalOption<'st> {
        for x in f.into_iter() {
            match x {
                EuType::Word(w) => {
                    if let e @ Some(_) = self.eval_word(&w.0) {
                        return e;
                    }
                }
                _ => self.stack.0.push(x),
            }
        }
        None
    }

    fn eval_word(&mut self, w: &str) -> EvalOption<'st> {
        if let Some((meta, f)) = CORE.get(w) {
            if self.stack.0.len() < meta.nargs {
                return self.err_nargs(meta);
            }
            f(self, meta)
        } else {
            Some(Box::new(format!("unknown word {}", w)))
        }
    }

    fn err_nargs(&self, meta: &EuFnMeta) -> EvalOption<'st> {
        Some(Box::new(format!(
            "(stack len) {} < {} ({})",
            self.stack.0.len(),
            meta.nargs,
            meta.name
        )))
    }
}
