use std::{
    iter::{
        self,
        Peekable,
    },
    mem,
};

use anyhow::{
    Context,
    anyhow,
};
use derive_more::Display;
use ecow::EcoVec;
use hipstr::HipStr;
use winnow::Parser;

use crate::{
    fns::CORE,
    parser::euphrates,
    types::{
        EuIter,
        EuRes,
        EuType,
    },
};

#[derive(Display)]
#[display("stack: {stack:?}\nscope: {scope:?}")]
pub struct EuEnv<'eu> {
    pub queue: Peekable<EuIter<'eu>>,
    pub stack: EcoVec<EuType<'eu>>,
    pub scope: EuScope<'eu>,
}

pub type EuScope<'eu> = imbl::HashMap<HipStr<'eu>, EuType<'eu>>;

impl<'eu> EuEnv<'eu> {
    #[inline]
    pub fn new<T>(ts: T, args: &[EuType<'eu>], scope: EuScope<'eu>) -> Self
    where
        T: IntoIterator<Item = EuType<'eu>>,
        T::IntoIter: 'eu,
    {
        let it: EuIter<'eu> = Box::new(ts.into_iter());
        Self {
            queue: it.peekable(),
            stack: args.into(),
            scope,
        }
    }

    #[inline]
    pub fn apply<T>(ts: T, args: &[EuType<'eu>], scope: EuScope<'eu>) -> EuRes<EuEnv<'eu>>
    where
        T: IntoIterator<Item = EuType<'eu>>,
        T::IntoIter: 'eu,
    {
        let mut env = Self::new(ts, args, scope);
        env.eval()?;
        Ok(env)
    }

    #[inline]
    pub fn apply_n_1<T>(ts: T, args: &[EuType<'eu>], scope: EuScope<'eu>) -> EuRes<EuType<'eu>>
    where
        T: IntoIterator<Item = EuType<'eu>>,
        T::IntoIter: 'eu,
    {
        Self::apply(ts, args, scope).and_then(|mut env| env.pop())
    }

    #[inline]
    pub fn run_str(input: &str) -> EuRes<Self> {
        let mut env = Self::str(input)?;
        env.eval()?;
        Ok(env)
    }

    #[inline]
    pub fn str(input: &str) -> EuRes<Self> {
        Ok(Self::new(
            euphrates.parse(input).map_err(|e| anyhow!(e.to_string()))?,
            &[],
            imbl::HashMap::new(),
        ))
    }

    pub fn eval(&mut self) -> EuRes<()> {
        while let Some(t) = self.queue.next() {
            println!("{t:?}\n>>>");
            self.eval_type(t)?;
            println!("{self}\n<<<");
        }
        Ok(())
    }

    fn eval_type(&mut self, t: EuType<'eu>) -> EuRes<()> {
        match t {
            EuType::Word(w) => self.eval_word(&w),
            EuType::Res(Err(e)) => Err(anyhow!(e.to_string()).into()),
            _ => {
                self.push(t);
                Ok(())
            }
        }
    }

    fn eval_word(&mut self, w: &str) -> EuRes<()> {
        if let Some(v) = self.scope.get(w) {
            if let EuType::Expr(ts) = v {
                self.eval_iter(ts.clone())
            } else {
                self.push(v.clone());
                Ok(())
            }
        } else if let Some(f) = CORE.get(w) {
            f(self)
                .with_context(|| format!("`{w}` failed"))
                .map_err(|e| e.into())
        } else {
            Err(anyhow!("unknown word `{w}`").into())
        }
    }

    #[inline]
    pub fn eval_iter<T>(&mut self, ts: T) -> EuRes<()>
    where
        T: IntoIterator<Item = EuType<'eu>>,
        T::IntoIter: 'eu,
    {
        if self.queue.peek().is_none() {
            self.load_iter(ts);
        } else {
            let mut env = self.frame(ts);
            env.eval()?;
            self.stack = env.stack;
        }
        Ok(())
    }

    #[inline]
    pub fn load_iter<T>(&mut self, ts: T)
    where
        T: IntoIterator<Item = EuType<'eu>>,
        T::IntoIter: 'eu,
    {
        let empty: EuIter<'eu> = Box::new(iter::empty());
        let it: EuIter<'eu> = Box::new(
            ts.into_iter()
                .chain(mem::replace(&mut self.queue, empty.peekable())),
        );
        self.queue = it.peekable();
    }

    #[inline]
    pub fn frame<T>(&self, ts: T) -> Self
    where
        T: IntoIterator<Item = EuType<'eu>>,
        T::IntoIter: 'eu,
    {
        let it: EuIter<'eu> = Box::new(ts.into_iter());
        Self {
            queue: it.peekable(),
            stack: self.stack.clone(),
            scope: self.scope.clone(),
        }
    }

    pub fn bind_args<T>(&mut self, ts: T) -> EuRes<()>
    where
        T: IntoIterator<Item = EuType<'eu>>,
        T::IntoIter: DoubleEndedIterator,
    {
        for t in ts.into_iter().rev() {
            let v = self.stack.pop().context("insufficient args passed")?;
            match t {
                EuType::Word(w) => self.scope.insert(w, v),
                _ => todo!(),
            };
        }
        Ok(())
    }

    #[inline]
    pub fn push(&mut self, t: EuType<'eu>) {
        self.stack.push(t);
    }

    #[inline]
    pub fn pop(&mut self) -> EuRes<EuType<'eu>> {
        self.check_nargs(1).map(|_| self.stack.pop().unwrap())
    }

    #[inline]
    pub fn last(&self) -> EuRes<&EuType<'eu>> {
        self.check_nargs(1).map(|_| self.stack.last().unwrap())
    }

    pub fn iflip(&self, i: isize) -> EuRes<usize> {
        let len = self.stack.len() as isize;
        let j = if i < 0 { !i } else { len - i - 1 };
        (0 <= j && j < len)
            .then_some(j as usize)
            .ok_or_else(|| anyhow!("{i} out of bounds [{}, {}]", -len, len - 1).into())
    }

    pub fn check_nargs(&self, n: usize) -> EuRes<()> {
        if self.stack.len() < n {
            Err(anyhow!("actual stack len {} < {} expected", self.stack.len(), n,).into())
        } else {
            Ok(())
        }
    }

    pub fn clear_queue(&mut self) {
        let queue: EuIter<'_> = Box::new(iter::empty());
        self.queue = queue.peekable();
    }
}
