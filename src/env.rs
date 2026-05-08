use std::{
    cell::RefCell,
    hash,
    iter::{
        self,
        Peekable,
    },
    mem,
    rc::Rc,
    sync::{
        Arc,
        atomic::{
            AtomicBool,
            Ordering,
        },
    },
};

use anyhow::{
    Context,
    anyhow,
};
use derive_more::{
    Debug,
    Display,
};
use ecow::EcoVec;
use hipstr::LocalHipStr;
use itertools::Itertools;
use rand::Rng;
use winnow::Parser;

use crate::{
    EuEnvOpts,
    fns::{
        CORE,
        bind,
    },
    parser::euphrates,
    types::{
        EuBind,
        EuIter,
        EuRes,
        EuSyn,
        EuType,
    },
};

#[derive(Debug, Display)]
#[debug("stack: {stack:?}\nscope: {scope:?}")]
#[display("{stack:?}")]
pub struct EuEnv<'eu> {
    pub queue: Peekable<EuIter<'eu>>,
    pub stack: EcoVec<EuType<'eu>>,
    pub scope: EuScope<'eu>,
    pub ctx: &'eu EuEnvCtx,
}

pub struct EuEnvCtx {
    pub opts: EuEnvOpts,
    pub interrupt: Arc<AtomicBool>,
    pub rng: RefCell<Box<dyn Rng>>,
}

pub type EuScope<'eu> =
    imbl::GenericHashMap<LocalHipStr<'eu>, EuType<'eu>, hash::RandomState, imbl::shared_ptr::RcK>;

impl<'eu> EuEnv<'eu> {
    pub fn new<T>(ts: T, args: &[EuType<'eu>], scope: EuScope<'eu>, ctx: &'eu EuEnvCtx) -> Self
    where
        T: IntoIterator<Item = EuSyn<'eu>>,
        T::IntoIter: 'eu,
    {
        let it: EuIter<'eu> = Box::new(ts.into_iter());
        Self {
            queue: it.peekable(),
            stack: args.into(),
            scope,
            ctx,
        }
    }

    #[inline]
    pub fn apply<T>(
        ts: T,
        args: &[EuType<'eu>],
        scope: EuScope<'eu>,
        ctx: &'eu EuEnvCtx,
    ) -> EuRes<EuEnv<'eu>>
    where
        T: IntoIterator<Item = EuSyn<'eu>>,
        T::IntoIter: 'eu,
    {
        let mut env = Self::new(ts, args, scope, ctx);
        env.eval()?;
        Ok(env)
    }

    #[inline]
    pub fn apply_n_1<T>(
        ts: T,
        args: &[EuType<'eu>],
        scope: EuScope<'eu>,
        ctx: &'eu EuEnvCtx,
    ) -> EuRes<EuType<'eu>>
    where
        T: IntoIterator<Item = EuSyn<'eu>>,
        T::IntoIter: 'eu,
    {
        Self::apply(ts, args, scope, ctx).and_then(|mut env| env.pop())
    }

    #[inline]
    pub fn apply_n_2<T>(
        ts: T,
        args: &[EuType<'eu>],
        scope: EuScope<'eu>,
        ctx: &'eu EuEnvCtx,
    ) -> EuRes<(EuType<'eu>, EuType<'eu>)>
    where
        T: IntoIterator<Item = EuSyn<'eu>>,
        T::IntoIter: 'eu,
    {
        Self::apply(ts, args, scope, ctx).and_then(|mut env| {
            env.check_nargs(2)?;
            #[expect(clippy::missing_panics_doc, reason = "infallible")]
            let a1 = env.stack.pop().unwrap();
            #[expect(clippy::missing_panics_doc, reason = "infallible")]
            let a0 = env.stack.pop().unwrap();
            Ok((a0, a1))
        })
    }

    #[inline]
    pub fn apply_str(
        s: &str,
        args: &[EuType<'eu>],
        scope: EuScope<'eu>,
        ctx: &'eu EuEnvCtx,
    ) -> EuRes<EuEnv<'eu>> {
        Self::apply(
            euphrates.parse(s).map_err(|e| anyhow!(e.to_string()))?,
            args,
            scope,
            ctx,
        )
    }

    pub fn eval(&mut self) -> EuRes<()> {
        while let Some(t) = self.queue.next() {
            #[cfg(not(target_arch = "wasm32"))]
            if !self.ctx.interrupt.load(Ordering::SeqCst) {
                return Err(anyhow!("interrupted").into());
            }
            if self.ctx.opts.debug {
                println!("{t:?}\n>>>");
            }
            self.eval_syn(t)?;
            if self.ctx.opts.debug {
                println!("{self:?}\n<<<\n");
            }
        }
        Ok(())
    }

    fn eval_syn(&mut self, t: EuSyn<'eu>) -> EuRes<()> {
        match t {
            EuSyn::Raw(t) => self.eval_type(t),
            EuSyn::Var(s) => self.eval_var(&s),
            EuSyn::Move(s) => self.eval_move(&s),
            EuSyn::Get(k) => self.eval_get(&k),
            EuSyn::Vec(ts) => {
                self.push(EuType::vec(
                    Self::apply(ts, &[], self.scope.clone(), self.ctx)?.stack,
                ));
                Ok(())
            }
            EuSyn::Map(ts) => {
                self.push(EuType::Map(Rc::new(
                    Self::apply(ts, &[], self.scope.clone(), self.ctx)?
                        .stack
                        .into_iter()
                        .map(EuType::to_pair)
                        .try_collect()?,
                )));
                Ok(())
            }
            EuSyn::Bind(bs) => self.bind_args(&bs),
        }
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
                .map_err(Into::into)
        } else {
            Err(anyhow!("unknown word `{w}`").into())
        }
    }

    fn eval_var(&mut self, w: &str) -> EuRes<()> {
        if let Some(v) = self.scope.get(w) {
            self.push(v.clone());
            Ok(())
        } else {
            Err(anyhow!("unknown var `{w}`").into())
        }
    }

    fn eval_move(&mut self, w: &str) -> EuRes<()> {
        if let Some(v) = self.scope.remove(w) {
            self.push(v);
            Ok(())
        } else {
            Err(anyhow!("unknown var `{w}`").into())
        }
    }

    fn eval_get(&mut self, w: &str) -> EuRes<()> {
        let ts = self
            .pop()?
            .get(&EuType::str(w))?
            .with_context(|| format!("missing key `{w}`"))?
            .to_expr()?;
        self.eval_iter(ts)
    }

    pub fn eval_iter<T>(&mut self, ts: T) -> EuRes<()>
    where
        T: IntoIterator<Item = EuSyn<'eu>>,
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

    pub fn load_iter<T>(&mut self, ts: T)
    where
        T: IntoIterator<Item = EuSyn<'eu>>,
        T::IntoIter: 'eu,
    {
        let empty: EuIter<'eu> = Box::new(iter::empty());
        let it: EuIter<'eu> = Box::new(
            ts.into_iter()
                .chain(mem::replace(&mut self.queue, empty.peekable())),
        );
        self.queue = it.peekable();
    }

    #[must_use]
    pub fn frame<T>(&self, ts: T) -> Self
    where
        T: IntoIterator<Item = EuSyn<'eu>>,
        T::IntoIter: 'eu,
    {
        let it: EuIter<'eu> = Box::new(ts.into_iter());
        Self {
            queue: it.peekable(),
            stack: self.stack.clone(),
            scope: self.scope.clone(),
            ctx: self.ctx,
        }
    }

    pub fn bind_args(&mut self, bs: &EcoVec<EuBind<'eu>>) -> EuRes<()> {
        for b in bs.iter().rev() {
            let t = self
                .stack
                .pop()
                .with_context(|| format!("missing `{b:?}`"))?;
            self.bind_type(b, t)?;
        }
        Ok(())
    }

    pub fn bind_type(&mut self, b: &EuBind<'eu>, t: EuType<'eu>) -> EuRes<()> {
        fn check<'eu>(a: &EuType<'eu>, b: &EuType<'eu>) -> EuRes<()> {
            (a == b).ok_or_else(|| anyhow!("expected `{a:?}`, got `{b:?}`").into())
        }

        match b {
            EuBind::Word(w) => {
                self.scope.insert(w.clone(), t);
            }

            EuBind::Tag(w, bs) => {
                if let Some(f) = bind::BIND.get(w) {
                    (f.bind)(self, bs, t).with_context(|| format!("in `${w}`"))?;
                } else {
                    return Err(anyhow!("unknown tag `${w}`").into());
                }
            }

            EuBind::Union(bs) => {
                let mut errs = EcoVec::new();
                for b in bs {
                    match self.bind_type(b, t.clone()) {
                        Ok(()) => return Ok(()),
                        Err(e) => errs.push(e),
                    }
                }
                return Err(
                    anyhow!("failed to bind `{bs:?}` ({})", errs.into_iter().join("; ")).into(),
                );
            }

            EuBind::Bind(b0, b1) => {
                self.bind_type(b0, t.clone())?;
                self.bind_type(b1, t)?;
            }

            EuBind::Bool(b) => check(&EuType::Bool(*b), &t)?,
            EuBind::I32(n) => check(&EuType::I32(*n), &t)?,
            EuBind::I64(n) => check(&EuType::I64(*n), &t)?,
            EuBind::IBig(n) => check(&EuType::IBig(n.clone()), &t)?,
            EuBind::F64(n) => check(&EuType::F64(*n), &t)?,
            EuBind::Char(c) => check(&EuType::Char(*c), &t)?,
            EuBind::Str(s) => check(&EuType::Str(s.clone()), &t)?,

            EuBind::Vecz(bs) => (bind::VECZ.bind)(self, bs, t)?,
            EuBind::Map(bs) => (bind::MAP.bind)(self, bs, t)?,
        }

        Ok(())
    }

    #[inline]
    pub fn push(&mut self, t: EuType<'eu>) {
        self.stack.push(t);
    }

    #[inline]
    pub fn pop(&mut self) -> EuRes<EuType<'eu>> {
        self.check_nargs(1).map(|()| {
            #[expect(clippy::missing_panics_doc, reason = "infallible")]
            self.stack.pop().unwrap()
        })
    }

    #[inline]
    pub fn arg(&mut self, name: &str) -> EuRes<EuType<'eu>> {
        Ok(self
            .stack
            .pop()
            .with_context(|| format!("missing arg `{name}`"))?)
    }

    #[inline]
    pub fn last(&self) -> EuRes<&EuType<'eu>> {
        self.check_nargs(1).map(|()| {
            #[expect(clippy::missing_panics_doc, reason = "infallible")]
            self.stack.last().unwrap()
        })
    }

    pub fn iflip(&self, i: isize) -> EuRes<usize> {
        let len = self.stack.len().cast_signed();
        let j = if i < 0 { !i } else { len - i - 1 };
        (0 <= j && j < len)
            .then_some(j.cast_unsigned())
            .ok_or_else(|| anyhow!("{i} out of bounds [{}, {}]", -len, len - 1).into())
    }

    pub fn check_nargs(&self, n: usize) -> EuRes<()> {
        if self.stack.len() < n {
            Err(anyhow!("actual stack len {} < {n} expected", self.stack.len()).into())
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn clear_queue(&mut self) {
        let queue: EuIter<'_> = Box::new(iter::empty());
        self.queue = queue.peekable();
    }
}

impl EuEnvCtx {
    pub fn new<R: Rng + 'static>(opts: EuEnvOpts, interrupt: Arc<AtomicBool>, rng: R) -> Self {
        EuEnvCtx {
            opts,
            interrupt,
            rng: RefCell::new(Box::new(rng)),
        }
    }
}
