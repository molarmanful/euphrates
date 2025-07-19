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
    types::EuType,
};

#[derive(Debug, Display, Clone)]
#[display("stack: {stack:?}\nscope: {scope:?}")]
pub struct EuState<'st> {
    pub stack: Vec<EuType<'st>>,
    pub scope: HashMap<&'st str, EuType<'st>>,
}

impl<'st> EuState<'st> {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
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

    fn eval_vec(&mut self, f: Vec<EuType<'st>>) -> EvalOption<'st> {
        for x in f.into_iter() {
            match x {
                EuType::Word(w) => {
                    if let e @ Some(_) = self.eval_word(&w) {
                        return e;
                    }
                }
                _ => self.stack.push(x),
            }
        }
        None
    }

    fn eval_word(&mut self, w: &str) -> EvalOption<'st> {
        if let Some((meta, f)) = CORE.get(w) {
            let meta = meta.name(w);
            if self.stack.len() < meta.nargs {
                return self.err_nargs(&meta);
            }
            f(self, &meta)
        } else {
            Some(Box::new(format!("unknown word {w}")))
        }
    }

    fn err_nargs(&self, meta: &EuFnMeta) -> EvalOption<'st> {
        Some(Box::new(format!(
            "(stack len) {} < {} ({})",
            self.stack.len(),
            meta.nargs,
            meta.name
        )))
    }

    pub fn iflip(&self, i: usize) -> usize {
        self.stack.len() - i - 1
    }
}
