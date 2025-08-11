use std::iter;

use itertools::Itertools;

use super::{
    EuRes,
    EuType,
};
use crate::{
    env::{
        EuEnv,
        EuScope,
    },
    utils::swap_errors,
};

impl<'eu> EuType<'eu> {
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
        F: FnMut(Self, Self) -> EuRes<Self> + Clone + 'eu,
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
        match self {
            Self::Opt(Some(t)) | Self::Res(Ok(t)) => f(init, *t),
            Self::Opt(None) | Self::Res(Err(_)) => Ok(init),
            _ => f(self, init),
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

    pub fn sorted(mut self) -> EuRes<Self> {
        match self {
            Self::Vec(ref mut ts) => {
                ts.make_mut().sort();
                Ok(self)
            }
            _ => Self::Vec(self.to_vec()?).sorted(),
        }
    }
}
