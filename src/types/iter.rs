use std::{
    cmp::{
        self,
        Ordering,
    },
    iter,
    mem,
    panic::{
        self,
        AssertUnwindSafe,
    },
    slice,
};

use ecow::EcoVec;
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
    types::EuSeqImpl,
    utils::{
        IterExt,
        swap_errors,
        unpanic,
    },
};

impl<'eu> EuType<'eu> {
    pub fn unfold<F>(mut self, mut f: F) -> impl EuSeqImpl<'eu>
    where
        F: FnMut(&Self) -> EuRes<(Self, Self)> + Clone + 'eu,
    {
        iter::from_fn(move || {
            f(&self)
                .map(|(st, t)| {
                    self = st;
                    t.to_opt()
                })
                .transpose()
        })
    }

    #[inline]
    pub fn unfold_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<impl EuSeqImpl<'eu>> {
        f.to_expr().map(|f| {
            self.unfold(move |acc| EuEnv::apply_n_2(f.clone(), slice::from_ref(acc), scope.clone()))
        })
    }

    #[inline]
    pub fn repeat(self) -> EuSeq<'eu> {
        Box::new(iter::repeat(Ok(self)))
    }

    #[inline]
    pub fn cycle(self) -> EuSeq<'eu> {
        match self {
            Self::Seq(it) => Box::new(it.cycle()),
            _ => Box::new(self.to_seq().cycle()),
        }
    }

    pub fn get_take(self, i: isize) -> EuRes<Option<Self>> {
        match self {
            Self::Opt(o) => Ok((i == 0).then(|| o.map(|t| *t)).flatten()),
            Self::Res(r) => Self::Opt(r.ok()).get_take(i),
            Self::Vec(mut ts) => Ok(if i < 0 {
                ts.len().checked_add_signed(i)
            } else {
                Some(i as usize)
            }
            .and_then(|i| {
                ts.make_mut()
                    .get_mut(i)
                    .map(|t| mem::replace(t, Self::Opt(None)))
            })),
            Self::Seq(mut it) => {
                if i < 0 {
                    Self::Vec(Self::Seq(it).to_vec()?).get_take(i)
                } else {
                    it.nth(i as usize).transpose()
                }
            }
            _ => Self::Vec(self.to_vec()?).get_take(i),
        }
    }

    pub fn take(self, n: isize) -> EuRes<Self> {
        let a = n.unsigned_abs();
        match self {
            Self::Opt(o) => Ok(Self::Opt((n != 0).then_some(o).flatten())),
            Self::Res(r) => Self::Opt(r.ok()).take(n),
            Self::Vec(ts) => Ok(if n < 0 {
                Self::vec(&ts[ts.len().saturating_sub(a)..])
            } else {
                Self::vec(&ts[..cmp::min(a, ts.len())])
            }),
            Self::Seq(it) => {
                if n < 0 {
                    Self::vec(Self::Seq(it).to_vec()?).take(n)
                } else {
                    Ok(Self::seq(it.take(a)))
                }
            }
            _ => Self::Vec(self.to_vec()?).take(n),
        }
    }

    pub fn drop(self, n: isize) -> EuRes<Self> {
        let a = n.unsigned_abs();
        match self {
            Self::Opt(o) => Ok(Self::Opt(if n != 0 { None } else { o })),
            Self::Res(r) => Self::Opt(r.ok()).drop(n),
            Self::Vec(ts) => Ok(if n < 0 {
                Self::vec(&ts[..ts.len().saturating_sub(a)])
            } else {
                Self::vec(&ts[cmp::min(a, ts.len())..])
            }),
            Self::Seq(it) => {
                if n < 0 {
                    Self::vec(Self::Seq(it).to_vec()?).drop(n)
                } else {
                    Ok(Self::seq(it.skip(a)))
                }
            }
            _ => Self::Vec(self.to_vec()?).take(n),
        }
    }

    pub fn chunk(self, n: isize) -> EuRes<Self> {
        let a = n.unsigned_abs();
        match self {
            Self::Opt(_) => Self::Vec(self.to_vec()?)
                .chunk(n)?
                .map(|t| t.get_take(0).map(Self::opt)),
            Self::Res(r) => Self::Opt(r.ok()).chunk(n),
            Self::Vec(ts) => {
                if n < 0 {
                    let l = ts.len() / a;
                    let mut r = ts.len() % a;
                    let mut c = a;
                    Ok(Self::Vec(
                        ts.into_iter()
                            .batching(move |it| {
                                if c == 0 {
                                    return None;
                                }
                                c -= 1;
                                let mut l = l;
                                if r > 0 {
                                    l += 1;
                                    r -= 1;
                                }
                                Some(Self::Vec(it.take(l).collect()))
                            })
                            .collect(),
                    ))
                } else {
                    Self::Seq(Self::Vec(ts).to_seq()).chunk(n)
                }
            }
            Self::Seq(it) => {
                if n < 0 {
                    Self::vec(Self::Seq(it).to_vec()?).chunk(n)
                } else {
                    Ok(Self::seq(it.batching(move |it| {
                        it.take(a)
                            .try_collect()
                            .map(|ts: EcoVec<_>| (a == 0 || !ts.is_empty()).then(|| Self::Vec(ts)))
                            .transpose()
                    })))
                }
            }
            _ => Self::Vec(self.to_vec()?).chunk(n),
        }
    }

    #[inline]
    pub fn window(self, n: usize) -> EuRes<Self> {
        self.divvy(n, 1)
    }

    pub fn divvy(self, n: usize, m: isize) -> EuRes<Self> {
        let a = m.unsigned_abs();
        match self {
            Self::Opt(_) => Self::Vec(self.to_vec()?)
                .divvy(n, m)?
                .map(|t| t.get_take(0).map(Self::opt)),
            Self::Res(r) => Self::Opt(r.ok()).divvy(n, m),
            Self::Vec(ref ts) => {
                if m < 0 {
                    let l = ts.len().saturating_sub(n).div_ceil(a);
                    self.divvy(n, l.try_into().unwrap())?
                        .to_vec()
                        .map(Self::Vec)
                } else {
                    Self::Seq(self.to_seq()).divvy(n, m)
                }
            }
            Self::Seq(it) => {
                if m < 0 {
                    Self::vec(Self::Seq(it).to_vec()?).divvy(n, m)
                } else {
                    let rem_empty = EcoVec::with_capacity(n.saturating_sub(a));
                    let mut rem = rem_empty.clone();
                    let mut first = true;
                    Ok(Self::seq(it.batching(move |it| {
                        if first {
                            first = false;
                        } else {
                            it.dropping(a.saturating_sub(n));
                        }
                        let n_ts = n - rem.len();
                        let r: Result<EcoVec<_>, _> = it.take(n_ts).try_collect();
                        match r {
                            Err(e) => Some(Err(e)),
                            Ok(ts) => (ts.len() >= n_ts).then(|| {
                                rem.extend(ts);
                                let res = mem::replace(&mut rem, rem_empty.clone());
                                rem.extend_from_slice(&res[cmp::min(a, n)..]);
                                Ok(Self::Vec(res))
                            }),
                        }
                    })))
                }
            }
            _ => Self::Vec(self.to_vec()?).divvy(n, m),
        }
    }

    pub fn multi_cartesian_product(ts: impl IntoIterator<Item = Self>) -> impl EuSeqImpl<'eu> {
        ts.into_iter()
            .map(Self::to_seq)
            .multi_cartesian_product()
            .map(|rs| rs.into_iter().try_collect().map(Self::Vec))
    }

    pub fn multi_zip(ts: impl IntoIterator<Item = Self>) -> impl EuSeqImpl<'eu> {
        let mut it = ts.into_iter().map(Self::to_seq).collect_vec();
        iter::from_fn(move || {
            it.iter_mut()
                .map(|it1| it1.next())
                .collect::<Option<Result<_, _>>>()
                .map(|r| r.map(Self::Vec))
        })
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
            Self::Seq(it) => Ok(Self::seq(it.flat_map(move |r| match r.and_then(&mut f) {
                Ok(t) => t.to_seq(),
                e => Box::new(iter::once(e)),
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

    pub fn flatten(self) -> EuRes<Self> {
        self.flat_map(Ok)
    }

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
                .filter_map(|t| f(&t).map(|b| b.then_some(t)).transpose())
                .try_collect()
                .map(Self::Vec),
            Self::Seq(it) => Ok(Self::seq(it.filter_map(move |r| {
                r.and_then(|t| f(&t).map(|b| b.then_some(t))).transpose()
            }))),
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
                    f(&t).map(|b| b.then_some(t)).transpose()
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

    pub fn take_while<F>(self, mut f: F) -> EuRes<Self>
    where
        F: FnMut(&Self) -> EuRes<bool> + Clone + 'eu,
    {
        match self {
            Self::Vec(ts) => ts
                .into_iter()
                .map_while(|t| f(&t).map(|b| b.then_some(t)).transpose())
                .try_collect()
                .map(Self::Vec),
            Self::Seq(it) => Ok(Self::seq(it.map_while(move |r| {
                r.and_then(|t| f(&t).map(|b| b.then_some(t))).transpose()
            }))),
            _ => self.take_while_once(f),
        }
    }

    pub fn take_while_once<F>(self, f: F) -> EuRes<Self>
    where
        F: FnOnce(&Self) -> EuRes<bool> + 'eu,
    {
        self.filter_once(f)
    }

    pub fn take_while_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        let f = f.to_expr()?;
        if self.is_many() {
            self.take_while(move |t| {
                EuEnv::apply_n_1(f.clone(), slice::from_ref(t), scope.clone()).map(Self::into)
            })
        } else {
            self.take_while_once(|t| EuEnv::apply_n_1(f, slice::from_ref(t), scope).map(Self::into))
        }
    }

    pub fn drop_while<F>(self, f: F) -> EuRes<Self>
    where
        F: FnMut(&Self) -> EuRes<bool> + Clone + 'eu,
    {
        match self {
            Self::Vec(ts) => ts
                .into_iter()
                .map(Ok)
                .skip_while_ok(f)
                .try_collect()
                .map(Self::Vec),
            Self::Seq(it) => Ok(Self::seq(it.skip_while_ok(f))),
            _ => self.drop_while_once(f),
        }
    }

    pub fn drop_while_once<F>(self, f: F) -> EuRes<Self>
    where
        F: FnOnce(&Self) -> EuRes<bool> + 'eu,
    {
        self.filter_once(|t| f(t).map(|b| !b))
    }

    pub fn drop_while_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        let f = f.to_expr()?;
        if self.is_many() {
            self.drop_while(move |t| {
                EuEnv::apply_n_1(f.clone(), slice::from_ref(t), scope.clone()).map(Self::into)
            })
        } else {
            self.drop_while_once(|t| EuEnv::apply_n_1(f, slice::from_ref(t), scope).map(Self::into))
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

    pub fn fold1<F>(self, mut f: F) -> EuRes<Option<Self>>
    where
        F: FnMut(Self, Self) -> EuRes<Self> + 'eu,
    {
        match self {
            Self::Vec(ts) => ts.into_iter().try_reduce(f),
            Self::Seq(it) => it.reduce(|a, b| f(a?, b?)).transpose(),
            _ => self.fold1_once(),
        }
    }

    #[inline]
    pub fn fold1_once(self) -> EuRes<Option<Self>> {
        Ok(self.to_opt())
    }

    pub fn fold1_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<Option<Self>> {
        let f = f.to_expr()?;
        if self.is_many() {
            self.fold1(move |a, b| EuEnv::apply_n_1(f.clone(), &[a, b], scope.clone()))
        } else {
            self.fold1_once()
        }
    }

    pub fn scan<F>(self, init: Self, mut f: F) -> EuRes<Self>
    where
        F: FnMut(&Self, Self) -> EuRes<(Self, Self)> + Clone + 'eu,
    {
        match self {
            Self::Vec(ts) => ts
                .into_iter()
                .scan(init, |acc, t| {
                    f(acc, t)
                        .map(|(st, t)| {
                            *acc = st;
                            t.to_opt()
                        })
                        .transpose()
                })
                .try_collect()
                .map(Self::Vec),
            Self::Seq(it) => Ok(Self::seq(it.scan(init, move |acc, t| {
                t.and_then(|t| f(acc, t))
                    .map(|(st, t)| {
                        *acc = st;
                        t.to_opt()
                    })
                    .transpose()
            }))),
            _ => self.scan_once(init, f),
        }
    }

    pub fn scan_once<F>(self, init: Self, f: F) -> EuRes<Self>
    where
        F: FnOnce(&Self, Self) -> EuRes<(Self, Self)> + 'eu,
    {
        match self {
            Self::Opt(Some(t)) => Ok(Self::opt(f(&init, *t)?.1.to_opt())),
            Self::Res(Ok(t)) => Ok(Self::res(match f(&init, *t)?.1 {
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
                EuEnv::apply_n_2(
                    f.clone(),
                    &[slice::from_ref(acc), &[t]].concat(),
                    scope.clone(),
                )
            })
        } else {
            self.scan_once(init, move |acc, t| {
                EuEnv::apply_n_2(f, &[slice::from_ref(acc), &[t]].concat(), scope)
            })
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
            _ => Self::Vec(self.to_vec()?).sorted_by(f),
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

    pub fn sorted_by_key<F>(mut self, mut f: F) -> EuRes<Self>
    where
        F: FnMut(&Self) -> EuRes<Self>,
    {
        match self {
            Self::Vec(ref mut ts) => {
                let res = unpanic(AssertUnwindSafe(|| {
                    ts.make_mut()
                        .sort_by_key(|t| f(t).unwrap_or_else(|e| panic::panic_any(e)))
                }));
                res.map(|()| self)
                    .map_err(|e| *e.downcast::<EuErr>().unwrap())
            }
            _ => Self::Vec(self.to_vec()?).sorted(),
        }
    }

    pub fn sorted_by_key_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        let f = f.to_expr()?;
        self.sorted_by_key(|t| EuEnv::apply_n_1(f.clone(), slice::from_ref(t), scope.clone()))
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
                    f(&t).map(|b| b.then_some(t)).transpose()
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

    pub fn any<F>(self, f: F) -> EuRes<bool>
    where
        F: FnMut(&Self) -> EuRes<bool> + 'eu,
    {
        self.find(f).map(|o| o.is_some())
    }

    pub fn any_once<F>(self, f: F) -> EuRes<bool>
    where
        F: FnOnce(&Self) -> EuRes<bool> + 'eu,
    {
        self.find_once(f).map(|o| o.is_some())
    }

    pub fn any_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<bool> {
        let f = f.to_expr()?;
        if self.is_many() {
            self.any(move |t| {
                EuEnv::apply_n_1(f.clone(), slice::from_ref(t), scope.clone()).map(Self::into)
            })
        } else {
            self.any_once(|t| EuEnv::apply_n_1(f, slice::from_ref(t), scope).map(Self::into))
        }
    }

    pub fn all<F>(self, mut f: F) -> EuRes<bool>
    where
        F: FnMut(&Self) -> EuRes<bool> + 'eu,
    {
        self.any(move |t| f(t).map(|b| !b)).map(|b| !b)
    }

    pub fn all_once<F>(self, f: F) -> EuRes<bool>
    where
        F: FnOnce(&Self) -> EuRes<bool> + 'eu,
    {
        self.any_once(|t| f(t).map(|b| !b)).map(|b| !b)
    }

    pub fn all_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<bool> {
        let f = f.to_expr()?;
        if self.is_many() {
            self.all(move |t| {
                EuEnv::apply_n_1(f.clone(), slice::from_ref(t), scope.clone()).map(Self::into)
            })
        } else {
            self.all_once(|t| EuEnv::apply_n_1(f, slice::from_ref(t), scope).map(Self::into))
        }
    }
}
