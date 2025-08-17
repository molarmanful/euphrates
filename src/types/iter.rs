use std::{
    cmp::{
        self,
        Ordering,
    },
    iter,
    panic::{
        self,
        AssertUnwindSafe,
    },
    slice,
};

use itertools::Itertools;

use super::{
    EuErr,
    EuRes,
    EuSeq,
    EuType,
};
use crate::{
    env::{
        EuEnv,
        EuScope,
    },
    utils::{
        swap_errors,
        unpanic,
    },
};

impl<'eu> EuType<'eu> {
    pub fn unfold<F>(mut self, mut f: F) -> EuSeq<'eu>
    where
        F: FnMut(Self) -> EuRes<(Self, Self)> + Clone + 'eu,
    {
        Box::new(iter::from_fn(move || match f(self.clone()) {
            Ok((st, t)) => {
                self = st;
                t.to_opt().map(Ok)
            }
            Err(e) => Some(Err(e)),
        }))
    }

    pub fn take(self, n: isize) -> EuRes<Self> {
        match self {
            Self::Opt(o) => Ok(Self::Opt(if n != 0 { o } else { None })),
            Self::Res(r) => Ok(Self::Res(if n != 0 {
                r
            } else {
                Err(Box::new(Self::Opt(None)))
            })),
            _ if self.is_vecz() => {
                let a = n.abs_diff(0);
                if n < 0 {
                    match self {
                        Self::Vec(ts) => Ok(Self::vec(&ts[ts.len().saturating_sub(a)..])),
                        Self::Seq(_) => Self::vec(self.to_vec()?).take(n),
                        _ => unreachable!(),
                    }
                } else {
                    match self {
                        Self::Vec(ts) => Ok(Self::vec(&ts[..cmp::min(a, ts.len())])),
                        Self::Seq(it) => Ok(Self::seq(it.take(a))),
                        _ => unreachable!(),
                    }
                }
            }
            _ => Self::Vec(self.to_vec()?).take(n),
        }
    }

    pub fn drop(self, n: isize) -> EuRes<Self> {
        match self {
            Self::Opt(o) => Ok(Self::Opt(if n != 0 { None } else { o })),
            Self::Res(r) => Ok(Self::Res(if n != 0 {
                Err(Box::new(Self::Opt(None)))
            } else {
                r
            })),
            _ if self.is_vecz() => {
                let a = n.abs_diff(0);
                if n < 0 {
                    match self {
                        Self::Vec(ts) => Ok(Self::vec(&ts[..ts.len().saturating_sub(a)])),
                        Self::Seq(_) => Self::vec(self.to_vec()?).drop(n),
                        _ => unreachable!(),
                    }
                } else {
                    match self {
                        Self::Vec(ts) => Ok(Self::vec(&ts[cmp::min(a, ts.len())..])),
                        Self::Seq(it) => Ok(Self::seq(it.skip(a))),
                        _ => unreachable!(),
                    }
                }
            }
            _ => Self::Vec(self.to_vec()?).take(n),
        }
    }

    pub fn map<F>(self, mut f: F) -> EuRes<Self>
    where
        F: FnMut(Self) -> EuRes<Self> + Clone + 'eu,
    {
        match self {
            Self::Vec(ts) => ts.into_iter().map(f).try_collect().map(Self::Vec),
            Self::Seq(it) => Ok(Self::seq(it.map(move |t| f(t?)))),
            _ => self.map_once(f),
        }
    }

    pub fn map_once<F>(self, f: F) -> EuRes<Self>
    where
        F: FnOnce(Self) -> EuRes<Self> + 'eu,
    {
        match self {
            Self::Opt(o) => o.map(|t| f(*t)).transpose().map(Self::opt),
            Self::Res(r) => swap_errors(r.map(|t| f(*t).map(Box::new))).map(Self::Res),
            _ => f(self),
        }
    }

    pub fn map_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        let f = f.to_expr()?;
        if self.is_many() {
            self.map(move |t| EuEnv::apply_n_1(f.clone(), &[t], scope.clone()))
        } else {
            self.map_once(|t| EuEnv::apply_n_1(f, &[t], scope))
        }
    }

    pub fn flat_map<F>(self, mut f: F) -> EuRes<Self>
    where
        F: FnMut(Self) -> EuRes<Self> + Clone + 'eu,
    {
        match self {
            Self::Vec(ts) => ts
                .into_iter()
                .flat_map(|t| match f(t) {
                    Ok(t) => t.to_seq(),
                    e @ Err(_) => Box::new(iter::once(e)),
                })
                .try_collect()
                .map(Self::Vec),
            Self::Seq(it) => Ok(Self::seq(it.flat_map(move |r| {
                match if let Ok(t) = r { f(t) } else { r } {
                    Ok(t) => t.to_seq(),
                    e => Box::new(iter::once(e)),
                }
            }))),
            _ => self.flat_map_once(f),
        }
    }

    pub fn flat_map_once<F>(self, f: F) -> EuRes<Self>
    where
        F: FnOnce(Self) -> EuRes<Self> + 'eu,
    {
        match self {
            Self::Opt(o) => o
                .and_then(|t| match f(*t) {
                    Ok(Self::Opt(o)) => o.map(Ok),
                    t => Some(t.map(Box::new)),
                })
                .transpose()
                .map(Self::Opt),
            Self::Res(r) => swap_errors(r.and_then(|t| match f(*t) {
                Ok(Self::Res(r)) => r.map(Ok),
                t => Ok(t.map(Box::new)),
            }))
            .map(Self::Res),
            _ => f(self),
        }
    }

    pub fn flat_map_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        let f = f.to_expr()?;
        if self.is_many() {
            self.flat_map(move |t| EuEnv::apply_n_1(f.clone(), &[t], scope.clone()))
        } else {
            self.flat_map_once(|t| EuEnv::apply_n_1(f, &[t], scope))
        }
    }

    #[inline]
    pub fn flatten(self) -> EuRes<Self> {
        self.flat_map(Ok)
    }

    #[inline]
    pub fn flatten_rec(self) -> EuRes<Self> {
        if self.is_vecz() {
            self.flat_map(|t| t.flatten_rec())
        } else {
            Ok(self)
        }
    }

    pub fn filter<F>(self, mut f: F) -> EuRes<Self>
    where
        F: FnMut(&Self) -> EuRes<bool> + Clone + 'eu,
    {
        match self {
            Self::Vec(ts) => ts
                .into_iter()
                .filter_map(|t| match f(&t) {
                    Ok(b) => b.then_some(Ok(t)),
                    Err(e) => Some(Err(e)),
                })
                .try_collect()
                .map(Self::Vec),
            Self::Seq(it) => {
                Ok(Self::seq(it.filter(move |t| {
                    t.as_ref().map(|t| f(t).unwrap_or(true)).unwrap_or(true)
                })))
            }
            _ => self.filter_once(f),
        }
    }

    pub fn filter_once<F>(self, f: F) -> EuRes<Self>
    where
        F: FnOnce(&Self) -> EuRes<bool> + 'eu,
    {
        match self {
            Self::Opt(o) => o
                .and_then(|t| {
                    let t = *t;
                    match f(&t) {
                        Ok(b) => b.then_some(Ok(t)),
                        Err(e) => Some(Err(e)),
                    }
                })
                .transpose()
                .map(Self::opt),
            Self::Res(r) => Self::Opt(r.ok()).filter_once(f),
            _ => Self::opt(self.to_opt()).filter_once(f),
        }
    }

    pub fn filter_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        let f = f.to_expr()?;
        if self.is_many() {
            self.filter(move |t| {
                EuEnv::apply_n_1(f.clone(), slice::from_ref(t), scope.clone()).map(Self::into)
            })
        } else {
            self.filter_once(|t| EuEnv::apply_n_1(f, slice::from_ref(t), scope).map(Self::into))
        }
    }

    pub fn zip<F>(self, t: Self, mut f: F) -> EuRes<Self>
    where
        F: FnMut(Self, Self) -> EuRes<Self> + Clone + 'eu,
    {
        match (self, t) {
            (Self::Vec(a), Self::Vec(b)) => a
                .into_iter()
                .zip(b)
                .map(|(a, b)| f(a, b))
                .try_collect()
                .map(Self::Vec),
            (Self::Seq(a), Self::Seq(b)) => {
                Ok(Self::seq(a.zip(b).map(move |(a, b)| {
                    a.and_then(|a| b.and_then(|b| f(a, b)))
                })))
            }
            (a, b) if a.is_vecz() => a.map(move |t| f(t, b.clone())),
            (a, b) if b.is_vecz() => b.map(move |t| f(a.clone(), t)),
            (a, b) => a.zip_once(b, f),
        }
    }

    pub fn zip_once<F>(self, t: Self, f: F) -> EuRes<Self>
    where
        F: FnOnce(Self, Self) -> EuRes<Self> + 'eu,
    {
        match (self, t) {
            (Self::Opt(a), Self::Opt(b)) => {
                a.zip(b).map(|(a, b)| f(*a, *b)).transpose().map(Self::opt)
            }
            (Self::Res(Ok(a)), Self::Res(Ok(b))) => f(*a, *b).map(|t| Self::res(Ok(t))),
            (Self::Res(a), Self::Res(b)) => Ok(Self::Res(a.and(b))),
            (a, b) if a.is_once() => a.map_once(|t| f(t, b)),
            (a, b) if b.is_once() => b.map_once(|t| f(a, t)),
            (a, b) => f(a, b),
        }
    }

    pub fn zip_env(self, t: Self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        let f = f.to_expr()?;
        if self.is_many() {
            self.zip(t, move |a, b| {
                EuEnv::apply_n_1(f.clone(), &[a, b], scope.clone())
            })
        } else {
            self.zip_once(t, |a, b| EuEnv::apply_n_1(f, &[a, b], scope))
        }
    }

    pub fn fold<F>(self, init: Self, mut f: F) -> EuRes<Self>
    where
        F: FnMut(Self, Self) -> EuRes<Self> + 'eu,
    {
        match self {
            Self::Vec(ts) => ts.into_iter().try_fold(init, f),
            Self::Seq(mut it) => it.try_fold(init, |acc, t| f(acc, t?)),
            _ => self.fold_once(init, f),
        }
    }

    pub fn fold_once<F>(self, init: Self, f: F) -> EuRes<Self>
    where
        F: FnOnce(Self, Self) -> EuRes<Self> + 'eu,
    {
        match self.to_opt() {
            Some(t) => f(init, t),
            None => Ok(init),
        }
    }

    pub fn fold_env(self, init: Self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        let f = f.to_expr()?;
        if self.is_many() {
            self.fold(init, move |acc, t| {
                EuEnv::apply_n_1(f.clone(), &[acc, t], scope.clone())
            })
        } else {
            self.fold_once(init, |acc, t| EuEnv::apply_n_1(f, &[acc, t], scope))
        }
    }

    pub fn scan<F>(self, init: Self, mut f: F) -> EuRes<Self>
    where
        F: FnMut(Self, Self) -> EuRes<(Self, Self)> + Clone + 'eu,
    {
        match self {
            Self::Vec(ts) => ts
                .into_iter()
                .scan(init, |acc, t| match f(acc.clone(), t) {
                    Ok((st, t)) => {
                        *acc = st;
                        t.to_opt().map(Ok)
                    }
                    Err(e) => Some(Err(e)),
                })
                .try_collect()
                .map(Self::Vec),
            Self::Seq(it) => {
                Ok(Self::seq(it.scan(
                    init,
                    move |acc, t| match t.and_then(|t| f(acc.clone(), t)) {
                        Ok((st, t)) => {
                            *acc = st;
                            t.to_opt().map(Ok)
                        }
                        Err(e) => Some(Err(e)),
                    },
                )))
            }
            _ => self.scan_once(init, f),
        }
    }

    pub fn scan_once<F>(self, init: Self, f: F) -> EuRes<Self>
    where
        F: FnOnce(Self, Self) -> EuRes<(Self, Self)> + 'eu,
    {
        match self {
            Self::Opt(Some(t)) => Ok(Self::opt(f(init, *t)?.1.to_opt())),
            Self::Res(Ok(t)) => Ok(Self::res(match f(init, *t)?.1 {
                Self::Opt(Some(t)) | Self::Res(Ok(t)) => Ok(*t),
                Self::Res(Err(e)) => Err(*e),
                e @ Self::Opt(None) => Err(e),
                t => Ok(t),
            })),
            Self::Opt(None) | Self::Res(Err(_)) => Ok(self),
            _ => Ok(self),
        }
    }

    pub fn scan_env(self, init: Self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        let f = f.to_expr()?;
        if self.is_many() {
            self.scan(init, move |acc, t| {
                EuEnv::apply_n_2(f.clone(), &[acc, t], scope.clone())
            })
        } else {
            self.scan_once(init, move |acc, t| EuEnv::apply_n_2(f, &[acc, t], scope))
        }
    }

    pub fn sorted(mut self) -> EuRes<Self> {
        match self {
            Self::Vec(ref mut ts) => {
                ts.make_mut().sort();
                Ok(self)
            }
            _ => Self::Vec(self.to_vec()?).sorted(),
        }
    }

    pub fn sorted_by<F>(mut self, mut f: F) -> EuRes<Self>
    where
        F: FnMut(&Self, &Self) -> EuRes<Ordering>,
    {
        match self {
            Self::Vec(ref mut ts) => {
                let res = unpanic(AssertUnwindSafe(|| {
                    ts.make_mut()
                        .sort_by(|a, b| f(a, b).unwrap_or_else(|e| panic::panic_any(e)))
                }));
                res.map(|()| self)
                    .map_err(|e| *e.downcast::<EuErr>().unwrap())
            }
            _ => Self::Vec(self.to_vec()?).sorted(),
        }
    }

    pub fn sorted_by_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        let f = f.to_expr()?;
        self.sorted_by(|a, b| {
            EuEnv::apply_n_1(
                f.clone(),
                &[slice::from_ref(a), slice::from_ref(b)].concat(),
                scope.clone(),
            )
            .map(|t| t.cmp(&Self::ibig(0)))
        })
    }

    pub fn find<F>(self, mut f: F) -> EuRes<Option<Self>>
    where
        F: FnMut(&Self) -> EuRes<bool> + 'eu,
    {
        match self {
            Self::Vec(ts) => ts.into_iter().try_find(f),
            Self::Seq(mut it) => it
                .try_find(|r| r.as_ref().map_err(|e| e.clone()).and_then(&mut f))
                .and_then(Option::transpose),
            _ => self.find_once(f),
        }
    }

    pub fn find_once<F>(self, f: F) -> EuRes<Option<Self>>
    where
        F: FnOnce(&Self) -> EuRes<bool> + 'eu,
    {
        match self {
            Self::Opt(o) => o
                .and_then(|t| {
                    let t = *t;
                    match f(&t) {
                        Ok(b) => b.then_some(Ok(t)),
                        Err(e) => Some(Err(e)),
                    }
                })
                .transpose(),
            Self::Res(r) => Self::Opt(r.ok()).find_once(f),
            _ => Self::opt(self.to_opt()).find_once(f),
        }
    }

    pub fn find_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<Option<Self>> {
        let f = f.to_expr()?;
        if self.is_many() {
            self.find(move |t| {
                EuEnv::apply_n_1(f.clone(), slice::from_ref(t), scope.clone()).map(Self::into)
            })
        } else {
            self.find_once(|t| EuEnv::apply_n_1(f, slice::from_ref(t), scope).map(Self::into))
        }
    }
}
