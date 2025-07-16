use std::{
    collections::HashMap,
    fmt,
};

use derive_more::Display;
use winnow::Parser as _;

use crate::{
    parser::euphrates,
    types::{
        EuType,
        EuVec,
    },
};

#[derive(Debug, Display, Clone)]
#[display("stack: {stack:?}\nscope: {scope:?}")]
pub struct EuState<'st> {
    stack: EuVec<'st>,
    scope: HashMap<&'st str, EuType<'st>>,
}

type EvalResult<'st> = Result<EuState<'st>, EvalError<'st>>;
type EvalOption<'e> = Option<EvalError<'e>>;
type EvalError<'e> = Box<dyn fmt::Display + 'e>;

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
            Ok(f) => self.eval_fn(f),
            Err(e) => Some(Box::new(e)),
        }
    }

    fn eval_fn(&mut self, f: EuVec<'st>) -> EvalOption<'st> {
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
        match w {
            "dup" => match &self.stack.0[..] {
                [.., x] => self.stack.0.push(x.clone()),
                _ => return self.err_nargs(w, 1),
            },
            "pop" => {
                if self.stack.0.pop().is_none() {
                    return self.err_nargs(w, 1);
                }
            }
            "+" => match &self.stack.0[..] {
                [.., EuType::I64(x), EuType::I64(y)] => {
                    self.stack.0.push(EuType::I64((x.0 + y.0).into()));
                }
                _ => return self.err_nargs(w, 2),
            },
            _ => return Some(Box::new("unimplemented")),
        }
        None
    }

    fn err_nargs(&self, w: &str, n: usize) -> EvalOption<'st> {
        Some(Box::new(format!(
            "(stack len) {} < {n} ({w})",
            self.stack.0.len()
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x() {
        match EuState::run("1pop dup +") {
            Ok(st) => println!("{}", st),
            Err(e) => panic!("{}", e),
        }
        panic!();
    }
}
