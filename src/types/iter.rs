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
    rc::Rc,
    slice,
};

use ecow::EcoVec;
use itertools::Itertools;
use ordermap::{
    OrderMap,
    OrderSet,
};

use super::{
    EuErr,
    EuRes,
    EuSeq,
    EuSeqImpl,
    EuSyn,
    EuType,
};
use crate::{
    env::{
        EuEnv,
        EuScope,
    },
    utils::{
        IterExt,
        swap_errors,
        unpanic,
    },
};

impl<'eu> EuType<'eu> {
    pub fn unfold<F>(mut self, mut f: F) -> impl EuSeqImpl<'eu>
    where
        F: FnMut(&mut Self) -> EuRes<(Self, Self)> + Clone + 'eu,
    {
        iter::from_fn(move || {
            f(&mut self)
                .map(|(st, t)| {
                    self = st;
                    t.to_opt()
                })
                .transpose()
        })
    }

    #[inline]
    pub fn unfold_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        f.vecz1(|f| {
            let f = f.to_expr()?;
            Ok(Self::seq(self.unfold(move |acc| {
                EuEnv::apply_n_2(f.clone(), slice::from_mut(acc), scope.clone())
            })))
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

    pub fn get_n_take(self, i: isize) -> EuRes<Option<Self>> {
        match self {
            Self::Opt(o) => Ok((i == 0).then(|| o.map(|t| *t)).flatten()),
            Self::Res(r) => Self::Opt(r.ok()).get_n_take(i),
            Self::Vec(mut ts) => Ok(if i < 0 {
                ts.len().checked_add_signed(i)
            } else {
                Some(i as usize)
            }
            .and_then(|i| ts.make_mut().get_mut(i).map(mem::take))),
            Self::Map(mut kvs) => Ok(if i < 0 {
                kvs.len().checked_add_signed(i)
            } else {
                Some(i as usize)
            }
            .and_then(|i| {
                Rc::make_mut(&mut kvs)
                    .get_index_mut(i)
                    .map(|(k, v)| Self::vec([k.clone(), mem::take(v)]))
            })),
            Self::Set(mut ts) => Ok(if i < 0 {
                ts.len().checked_add_signed(i)
            } else {
                Some(i as usize)
            }
            .and_then(|i| Rc::make_mut(&mut ts).get_index(i).cloned())),
            Self::Seq(mut it) => {
                if i < 0 {
                    Self::Vec(Self::Seq(it).to_vec()?).get_n_take(i)
                } else {
                    it.nth(i as usize).transpose()
                }
            }
            _ => Self::Vec(self.to_vec()?).get_n_take(i),
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
            Self::Map(kvs) => Ok(Self::Map(Rc::new({
                let it = Rc::unwrap_or_clone(kvs).into_iter();
                if n < 0 {
                    it.rev().take(a).rev().collect()
                } else {
                    it.take(a).collect()
                }
            }))),
            Self::Set(ts) => Ok(Self::Set(Rc::new({
                let it = Rc::unwrap_or_clone(ts).into_iter();
                if n < 0 {
                    it.rev().take(a).rev().collect()
                } else {
                    it.take(a).collect()
                }
            }))),
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
            Self::Map(kvs) => Ok(Self::Map(Rc::new({
                let it = Rc::unwrap_or_clone(kvs).into_iter();
                if n < 0 {
                    it.rev().skip(a).rev().collect()
                } else {
                    it.skip(a).collect()
                }
            }))),
            Self::Set(ts) => Ok(Self::Set(Rc::new({
                let it = Rc::unwrap_or_clone(ts).into_iter();
                if n < 0 {
                    it.rev().skip(a).rev().collect()
                } else {
                    it.skip(a).collect()
                }
            }))),
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

        fn split<T, U, V, W>(
            ts: impl IntoIterator<Item = T>,
            len: usize,
            a: usize,
            inner: impl Fn(U) -> V,
        ) -> W
        where
            U: FromIterator<T>,
            W: FromIterator<V>,
        {
            let l = len / a;
            let mut r = len % a;
            let mut c = a;
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
                    Some(inner(it.take(l).collect()))
                })
                .collect()
        }

        match self {
            Self::Opt(_) => Self::Vec(self.to_vec()?)
                .chunk(n)?
                .map(|t| t.get_n_take(0).map(Self::opt)),
            Self::Res(r) => Self::Opt(r.ok()).chunk(n),
            Self::Vec(ts) => {
                if n < 0 {
                    let len = ts.len();
                    Ok(Self::Vec(split(ts, len, a, Self::Vec)))
                } else {
                    Self::Seq(Self::Vec(ts).to_seq()).chunk(n)
                }
            }
            Self::Map(kvs) => Ok(if n < 0 {
                let len = kvs.len();
                Self::Vec(split(Rc::unwrap_or_clone(kvs), len, a, |kvs| {
                    Self::Map(Rc::new(kvs))
                }))
            } else {
                Self::seq(Rc::unwrap_or_clone(kvs).into_iter().batching(move |it| {
                    let kvs: OrderMap<_, _> = it.take(a).collect();
                    (a == 0 || !kvs.is_empty()).then_some(Ok(Self::map_(kvs)))
                }))
            }),
            Self::Set(ts) => Ok(if n < 0 {
                let len = ts.len();
                Self::Vec(split(Rc::unwrap_or_clone(ts), len, a, |ts| {
                    Self::Set(Rc::new(ts))
                }))
            } else {
                Self::seq(Rc::unwrap_or_clone(ts).into_iter().batching(move |it| {
                    let ts: OrderSet<_, _> = it.take(a).collect();
                    (a == 0 || !ts.is_empty()).then_some(Ok(Self::set(ts)))
                }))
            }),
            Self::Seq(it) => {
                if n < 0 {
                    Self::vec(Self::Seq(it).to_vec()?).chunk(n)
                } else {
                    Ok(Self::seq(it.batching(move |it| {
                        it.take(a)
                            .try_collect()
                            .map(|ts: EcoVec<_>| {
                                (a == 0 || !ts.is_empty()).then_some(Self::Vec(ts))
                            })
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
                .map(|t| t.get_n_take(0).map(Self::opt)),
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
            Self::Map(kvs) => {
                if m < 0 {
                    let l = kvs.len().saturating_sub(n).div_ceil(a);
                    Self::Map(kvs)
                        .divvy(n, l.try_into().unwrap())?
                        .to_vec()
                        .map(Self::Vec)
                } else {
                    let rem_empty = OrderMap::with_capacity(n.saturating_sub(a));
                    let mut rem = rem_empty.clone();
                    let mut first = true;
                    Ok(Self::seq(Rc::unwrap_or_clone(kvs).into_iter().batching(
                        move |it| {
                            if first {
                                first = false;
                            } else {
                                it.dropping(a.saturating_sub(n));
                            }
                            let n_kvs = n - rem.len();
                            let kvs: OrderMap<_, _> = it.take(n_kvs).collect();
                            (kvs.len() >= n_kvs).then(|| {
                                rem.extend(kvs);
                                let res = mem::replace(&mut rem, rem_empty.clone());
                                rem.extend(res.iter().skip(a).map(|(k, v)| (k.clone(), v.clone())));
                                Ok(Self::map_(res))
                            })
                        },
                    )))
                }
            }
            Self::Set(ts) => {
                if m < 0 {
                    let l = ts.len().saturating_sub(n).div_ceil(a);
                    Self::Set(ts)
                        .divvy(n, l.try_into().unwrap())?
                        .to_vec()
                        .map(Self::Vec)
                } else {
                    let rem_empty = OrderSet::with_capacity(n.saturating_sub(a));
                    let mut rem = rem_empty.clone();
                    let mut first = true;
                    Ok(Self::seq(Rc::unwrap_or_clone(ts).into_iter().batching(
                        move |it| {
                            if first {
                                first = false;
                            } else {
                                it.dropping(a.saturating_sub(n));
                            }
                            let n_ts = n - rem.len();
                            let ts: OrderSet<_> = it.take(n_ts).collect();
                            (ts.len() >= n_ts).then(|| {
                                rem.extend(ts);
                                let res = mem::replace(&mut rem, rem_empty.clone());
                                rem.extend(res.iter().skip(a).cloned());
                                Ok(Self::set(res))
                            })
                        },
                    )))
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
                        it.take(n_ts)
                            .try_collect::<_, EcoVec<_>, _>()
                            .map(|ts| {
                                (ts.len() >= n_ts).then(|| {
                                    rem.extend(ts);
                                    let res = mem::replace(&mut rem, rem_empty.clone());
                                    rem.extend(res.iter().skip(a).cloned());
                                    Self::Vec(res)
                                })
                            })
                            .transpose()
                    })))
                }
            }
            _ => Self::Vec(self.to_vec()?).divvy(n, m),
        }
    }

    pub fn enumerate(self) -> Self {
        match self {
            Self::Seq(it) => Self::seq(it.enumerate().map(Self::enum_to_pair)),
            _ => Self::seq(self.to_seq()).enumerate(),
        }
    }

    fn enum_to_pair((i, r): (usize, EuRes<Self>)) -> EuRes<Self> {
        r.map(|t| Self::vec([Self::I64(i as i64), t]))
    }

    pub fn pairs(self) -> Self {
        match self {
            Self::Seq(it) => Self::seq(it.map(|r| r?.to_pair().map(|(a, b)| Self::vec([a, b])))),
            _ => Self::seq(self.to_seq()).pairs(),
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

    pub fn for_rec<F>(self, f: &mut F) -> EuRes<()>
    where
        F: FnMut(EcoVec<EuSyn<'eu>>) -> EuRes<()>,
    {
        if self.is_vecz() {
            for t in self.to_seq() {
                t?.for_rec(f)?;
            }
            Ok(())
        } else {
            f(self.to_expr()?)
        }
    }

    pub fn vecz1<F>(self, f: F) -> EuRes<Self>
    where
        F: FnOnce(Self) -> EuRes<Self> + Clone + 'eu,
    {
        if self.is_many() {
            self.map(move |t| t.vecz1(f.clone()))
        } else if self.is_once() {
            self.map_once(|t| t.vecz1(f))
        } else {
            f(self)
        }
    }

    pub fn vecz2<F>(self, t: Self, f_env: F) -> EuRes<Self>
    where
        F: FnOnce(Self, Self) -> EuRes<Self> + Clone + 'eu,
    {
        if self.is_many() || t.is_many() {
            self.zip(t, move |a, b| a.vecz2(b, f_env.clone()))
        } else if self.is_once() {
            self.zip_once(t, |a, b| a.vecz2(b, f_env))
        } else {
            f_env(self, t)
        }
    }

    pub fn map<F>(self, mut f: F) -> EuRes<Self>
    where
        F: FnMut(Self) -> EuRes<Self> + Clone + 'eu,
    {
        match self {
            Self::Vec(ts) => ts.into_iter().map(f).try_collect().map(Self::Vec),
            Self::Map(kvs) => Rc::unwrap_or_clone(kvs)
                .into_iter()
                .map(|(k, v)| f(v).map(|v| (k, v)))
                .try_collect()
                .map(|kvs| Self::Map(Rc::new(kvs))),
            Self::Set(ts) => Rc::unwrap_or_clone(ts)
                .into_iter()
                .map(f)
                .try_collect()
                .map(|ts| Self::Set(Rc::new(ts))),
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
        f.vecz1(|f| {
            let f = f.to_expr()?;
            if self.is_many() {
                self.map(move |t| EuEnv::apply_n_1(f.clone(), &[t], scope.clone()))
            } else {
                self.map_once(|t| EuEnv::apply_n_1(f, &[t], scope))
            }
        })
    }

    pub fn map_atom_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        f.vecz1(|f| {
            let f = f.to_expr()?;
            self.vecz1(|t| EuEnv::apply_n_1(f, &[t], scope))
        })
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
                    e => Box::new(iter::once(e)),
                })
                .try_collect()
                .map(Self::Vec),
            Self::Map(kvs) => Rc::unwrap_or_clone(kvs)
                .into_values()
                .flat_map(|t| match f(t) {
                    Ok(t) => t.to_seq(),
                    e => Box::new(iter::once(e)),
                })
                .try_collect()
                .map(Self::Vec),
            Self::Set(ts) => Rc::unwrap_or_clone(ts)
                .into_iter()
                .flat_map(|t| match f(t) {
                    Ok(t) => t.to_seq(),
                    e => Box::new(iter::once(e)),
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
        f.vecz1(|f| {
            let f = f.to_expr()?;
            if self.is_many() {
                self.flat_map(move |t| EuEnv::apply_n_1(f.clone(), &[t], scope.clone()))
            } else {
                self.flat_map_once(|t| EuEnv::apply_n_1(f, &[t], scope))
            }
        })
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
            Self::Map(kvs) => Rc::unwrap_or_clone(kvs)
                .into_iter()
                .filter_map(|kv| f(&kv.1).map(|b| b.then_some(kv)).transpose())
                .try_collect()
                .map(|kvs| Self::Map(Rc::new(kvs))),
            Self::Set(ts) => Rc::unwrap_or_clone(ts)
                .into_iter()
                .filter_map(|t| f(&t).map(|b| b.then_some(t)).transpose())
                .try_collect()
                .map(|ts| Self::Set(Rc::new(ts))),
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
        f.vecz1(|f| {
            let f = f.to_expr()?;
            if self.is_many() {
                self.filter(move |t| {
                    EuEnv::apply_n_1(f.clone(), slice::from_ref(t), scope.clone()).map(Self::into)
                })
            } else {
                self.filter_once(|t| EuEnv::apply_n_1(f, slice::from_ref(t), scope).map(Self::into))
            }
        })
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
            Self::Map(kvs) => Rc::unwrap_or_clone(kvs)
                .into_iter()
                .map_while(|kv| f(&kv.1).map(|b| b.then_some(kv)).transpose())
                .try_collect()
                .map(|kvs| Self::Map(Rc::new(kvs))),
            Self::Set(ts) => Rc::unwrap_or_clone(ts)
                .into_iter()
                .map_while(|t| f(&t).map(|b| b.then_some(t)).transpose())
                .try_collect()
                .map(|kvs| Self::Set(Rc::new(kvs))),
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
        f.vecz1(|f| {
            let f = f.to_expr()?;
            if self.is_many() {
                self.take_while(move |t| {
                    EuEnv::apply_n_1(f.clone(), slice::from_ref(t), scope.clone()).map(Self::into)
                })
            } else {
                self.take_while_once(|t| {
                    EuEnv::apply_n_1(f, slice::from_ref(t), scope).map(Self::into)
                })
            }
        })
    }

    pub fn drop_while<F>(self, mut f: F) -> EuRes<Self>
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
            Self::Map(kvs) => Rc::unwrap_or_clone(kvs)
                .into_iter()
                .map(Ok)
                .skip_while_ok(|(_, v)| f(v))
                .try_collect()
                .map(|kvs| Self::Map(Rc::new(kvs))),
            Self::Set(ts) => Rc::unwrap_or_clone(ts)
                .into_iter()
                .map(Ok)
                .skip_while_ok(f)
                .try_collect()
                .map(|kvs| Self::Set(Rc::new(kvs))),
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
        f.vecz1(|f| {
            let f = f.to_expr()?;
            if self.is_many() {
                self.drop_while(move |t| {
                    EuEnv::apply_n_1(f.clone(), slice::from_ref(t), scope.clone()).map(Self::into)
                })
            } else {
                self.drop_while_once(|t| {
                    EuEnv::apply_n_1(f, slice::from_ref(t), scope).map(Self::into)
                })
            }
        })
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
            (Self::Map(a), Self::Map(b)) => Rc::unwrap_or_clone(a)
                .into_values()
                .zip(Rc::unwrap_or_clone(b).into_values())
                .map(|(a, b)| f(a, b))
                .try_collect()
                .map(Self::Vec),
            (Self::Set(a), Self::Set(b)) => Rc::unwrap_or_clone(a)
                .into_iter()
                .zip(Rc::unwrap_or_clone(b))
                .map(|(a, b)| f(a, b))
                .try_collect()
                .map(Self::Vec),
            (Self::Seq(a), Self::Seq(b)) => {
                Ok(Self::seq(a.zip(b).map(move |(a, b)| {
                    a.and_then(|a| b.and_then(|b| f(a, b)))
                })))
            }
            (a, b) if a.is_many() && b.is_many() => {
                if a.is_seq() || b.is_seq() {
                    Self::Seq(a.to_seq()).zip(Self::Seq(b.to_seq()), f)
                } else if a.is_map() || b.is_map() {
                    Self::Map(a.to_map()?).zip(Self::Map(b.to_map()?), f)
                } else {
                    Self::Vec(a.to_vec()?).zip(Self::Vec(b.to_vec()?), f)
                }
            }
            (a, b) if a.is_many() => a.map(move |t| f(t, b.clone())),
            (a, b) if b.is_many() => b.map(move |t| f(a.clone(), t)),
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
        f.vecz1(|f| {
            let f = f.to_expr()?;
            if self.is_many() || t.is_many() {
                self.zip(t, move |a, b| {
                    EuEnv::apply_n_1(f.clone(), &[a, b], scope.clone())
                })
            } else {
                self.zip_once(t, |a, b| EuEnv::apply_n_1(f, &[a, b], scope))
            }
        })
    }

    pub fn zip_atom_env(self, t: Self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        f.vecz1(|f| {
            let f = f.to_expr()?;
            self.vecz2(t, |a, b| EuEnv::apply_n_1(f, &[a, b], scope))
        })
    }

    pub fn fold<F>(self, init: Self, mut f: F) -> EuRes<Self>
    where
        F: FnMut(Self, Self) -> EuRes<Self> + 'eu,
    {
        match self {
            Self::Vec(ts) => ts.into_iter().try_fold(init, f),
            Self::Map(kvs) => Rc::unwrap_or_clone(kvs).into_values().try_fold(init, f),
            Self::Set(ts) => Rc::unwrap_or_clone(ts).into_iter().try_fold(init, f),
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
        f.vecz1(|f| {
            let f = f.to_expr()?;
            if self.is_many() {
                self.fold(init, move |acc, t| {
                    EuEnv::apply_n_1(f.clone(), &[acc, t], scope.clone())
                })
            } else {
                self.fold_once(init, |acc, t| EuEnv::apply_n_1(f, &[acc, t], scope))
            }
        })
    }

    pub fn fold1<F>(self, mut f: F) -> EuRes<Option<Self>>
    where
        F: FnMut(Self, Self) -> EuRes<Self> + 'eu,
    {
        match self {
            Self::Vec(ts) => ts.into_iter().try_reduce(f),
            Self::Map(kvs) => Rc::unwrap_or_clone(kvs).into_values().try_reduce(f),
            Self::Set(ts) => Rc::unwrap_or_clone(ts).into_iter().try_reduce(f),
            Self::Seq(it) => it.reduce(|a, b| f(a?, b?)).transpose(),
            _ => self.fold1_once(),
        }
    }

    #[inline]
    pub fn fold1_once(self) -> EuRes<Option<Self>> {
        Ok(self.to_opt())
    }

    pub fn fold1_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        f.vecz1(|f| {
            let f = f.to_expr()?;
            if self.is_many() {
                self.fold1(move |a, b| EuEnv::apply_n_1(f.clone(), &[a, b], scope.clone()))
            } else {
                self.fold1_once()
            }
            .map(Self::opt)
        })
    }

    pub fn scan<F>(self, init: Self, mut f: F) -> EuRes<Self>
    where
        F: FnMut(&mut Self, Self) -> EuRes<(Self, Self)> + Clone + 'eu,
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
            Self::Map(kvs) => Rc::unwrap_or_clone(kvs)
                .into_values()
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
            Self::Set(ts) => Rc::unwrap_or_clone(ts)
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

    pub fn scan_once<F>(self, mut init: Self, f: F) -> EuRes<Self>
    where
        F: FnOnce(&mut Self, Self) -> EuRes<(Self, Self)> + 'eu,
    {
        match self {
            Self::Opt(Some(t)) => Ok(Self::opt(f(&mut init, *t)?.1.to_opt())),
            Self::Res(Ok(t)) => Ok(Self::res(match f(&mut init, *t)?.1 {
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
        f.vecz1(|f| {
            let f = f.to_expr()?;
            if self.is_many() {
                self.scan(init, move |acc, t| {
                    EuEnv::apply_n_2(f.clone(), &[mem::take(acc), t], scope.clone())
                })
            } else {
                self.scan_once(init, move |acc, t| {
                    EuEnv::apply_n_2(f, &[mem::take(acc), t], scope)
                })
            }
        })
    }

    pub fn sorted(mut self) -> EuRes<Self> {
        match self {
            Self::Vec(ref mut ts) => {
                ts.make_mut().sort();
                Ok(self)
            }
            Self::Map(ref mut kvs) => {
                Rc::make_mut(kvs).sort_unstable_keys();
                Ok(self)
            }
            Self::Set(ref mut ts) => {
                Rc::make_mut(ts).sort_unstable();
                Ok(self)
            }
            Self::Seq(it) => it.sorted().try_collect().map(Self::Vec),
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
            Self::Map(ref mut kvs) => {
                let res = unpanic(AssertUnwindSafe(|| {
                    Rc::make_mut(kvs).sort_by(|k0, v0, k1, v1| {
                        f(
                            &Self::vec([k0.clone(), v0.clone()]),
                            &Self::vec([k1.clone(), v1.clone()]),
                        )
                        .unwrap_or_else(|e| panic::panic_any(e))
                    })
                }));
                res.map(|()| self)
                    .map_err(|e| *e.downcast::<EuErr>().unwrap())
            }
            Self::Set(ref mut ts) => {
                let res = unpanic(AssertUnwindSafe(|| {
                    Rc::make_mut(ts).sort_by(|a, b| f(a, b).unwrap_or_else(|e| panic::panic_any(e)))
                }));
                res.map(|()| self)
                    .map_err(|e| *e.downcast::<EuErr>().unwrap())
            }
            Self::Seq(it) => unpanic(AssertUnwindSafe(|| {
                it.sorted_by(|a, b| {
                    f(
                        a.as_ref().unwrap_or_else(|e| panic::panic_any(e.clone())),
                        b.as_ref().unwrap_or_else(|e| panic::panic_any(e.clone())),
                    )
                    .unwrap_or_else(|e| panic::panic_any(e))
                })
                .try_collect()
                .map(Self::Vec)
                .unwrap_or_else(|e| panic::panic_any(e))
            }))
            .map_err(|e| *e.downcast::<EuErr>().unwrap()),
            _ => Self::Vec(self.to_vec()?).sorted_by(f),
        }
    }

    pub fn sorted_by_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        f.vecz1(|f| {
            let f = f.to_expr()?;
            self.sorted_by(move |a, b| {
                EuEnv::apply_n_1(f.clone(), &[a.clone(), b.clone()], scope.clone())
                    .map(|t| t.cmp(&Self::ibig(0)))
            })
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
            Self::Map(ref mut kvs) => {
                let res = unpanic(AssertUnwindSafe(|| {
                    Rc::make_mut(kvs).sort_by_key(|k, v| {
                        f(&Self::vec([k.clone(), v.clone()]))
                            .unwrap_or_else(|e| panic::panic_any(e))
                    })
                }));
                res.map(|()| self)
                    .map_err(|e| *e.downcast::<EuErr>().unwrap())
            }
            Self::Set(ref mut ts) => {
                let res = unpanic(AssertUnwindSafe(|| {
                    Rc::make_mut(ts).sort_by_key(|t| f(t).unwrap_or_else(|e| panic::panic_any(e)))
                }));
                res.map(|()| self)
                    .map_err(|e| *e.downcast::<EuErr>().unwrap())
            }
            Self::Seq(it) => unpanic(AssertUnwindSafe(|| {
                it.sorted_by_key(|t| {
                    f(t.as_ref().unwrap_or_else(|e| panic::panic_any(e.clone())))
                        .unwrap_or_else(|e| panic::panic_any(e))
                })
                .try_collect()
                .map(Self::Vec)
                .unwrap_or_else(|e| panic::panic_any(e))
            }))
            .map_err(|e| *e.downcast::<EuErr>().unwrap()),
            _ => Self::Vec(self.to_vec()?).sorted(),
        }
    }

    pub fn sorted_by_key_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        f.vecz1(move |f| {
            let f = f.to_expr()?;
            self.sorted_by_key(|t| EuEnv::apply_n_1(f.clone(), slice::from_ref(t), scope.clone()))
        })
    }

    pub fn find<F>(self, mut f: F) -> EuRes<Option<Self>>
    where
        F: FnMut(&Self) -> EuRes<bool> + 'eu,
    {
        match self {
            Self::Vec(ts) => ts.into_iter().try_find(f),
            Self::Map(kvs) => Rc::unwrap_or_clone(kvs).into_values().try_find(f),
            Self::Set(ts) => Rc::unwrap_or_clone(ts).into_iter().try_find(f),
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

    pub fn find_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        f.vecz1(|f| {
            let f = f.to_expr()?;
            if self.is_many() {
                self.find(move |t| {
                    EuEnv::apply_n_1(f.clone(), slice::from_ref(t), scope.clone()).map(Self::into)
                })
            } else {
                self.find_once(|t| EuEnv::apply_n_1(f, slice::from_ref(t), scope).map(Self::into))
            }
            .map(Self::opt)
        })
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

    pub fn any_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        f.vecz1(|f| {
            let f = f.to_expr()?;
            if self.is_many() {
                self.any(move |t| {
                    EuEnv::apply_n_1(f.clone(), slice::from_ref(t), scope.clone()).map(Self::into)
                })
            } else {
                self.any_once(|t| EuEnv::apply_n_1(f, slice::from_ref(t), scope).map(Self::into))
            }
            .map(Self::Bool)
        })
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

    pub fn all_env(self, f: Self, scope: EuScope<'eu>) -> EuRes<Self> {
        f.vecz1(|f| {
            let f = f.to_expr()?;
            if self.is_many() {
                self.all(move |t| {
                    EuEnv::apply_n_1(f.clone(), slice::from_ref(t), scope.clone()).map(Self::into)
                })
            } else {
                self.all_once(|t| EuEnv::apply_n_1(f, slice::from_ref(t), scope).map(Self::into))
            }
            .map(Self::Bool)
        })
    }
}
