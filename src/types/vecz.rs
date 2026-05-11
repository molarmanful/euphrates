use std::{
    iter::{
        self,
        Iterator,
    },
    mem,
    rc::Rc,
};

use anyhow::anyhow;
use hipstr::LocalHipStr;
use itertools::Itertools;

use crate::types::{
    EuRes,
    EuSyn,
    EuType,
};

impl EuType<'_> {
    pub fn get(self, t: &Self) -> EuRes<Option<Self>> {
        match self {
            Self::Map(mut kvs) => Ok(Rc::make_mut(&mut kvs).get_mut(t).map(mem::take)),
            _ => t
                .to_isize()
                .and_then(|i| self.at(i).transpose())
                .transpose(),
        }
    }

    pub fn at(self, index: isize) -> EuRes<Option<Self>> {
        let check = |len: usize| {
            if index < 0 {
                len.checked_add_signed(index)
            } else {
                Some(index.cast_unsigned())
            }
        };

        match self {
            Self::Opt(o) => Ok((index == 0 || index == -1).then(|| o.map(|t| *t)).flatten()),
            Self::Res(r) => Self::Opt(r.ok()).at(index),
            Self::Vec(mut ts) => {
                Ok(check(ts.len()).and_then(|i| ts.make_mut().get_mut(i).map(mem::take)))
            }
            Self::Map(mut kvs) => Ok(check(kvs.len()).and_then(|i| {
                Rc::make_mut(&mut kvs)
                    .get_index_mut(i)
                    .map(|(k, v)| Self::vec([k.clone(), mem::take(v)]))
            })),
            Self::Set(mut ts) => {
                Ok(check(ts.len()).and_then(|i| Rc::make_mut(&mut ts).get_index(i).cloned()))
            }
            Self::Seq(mut it) => {
                if index < 0 {
                    Self::Vec(Self::Seq(it).to_vec()?).at(index)
                } else {
                    it.nth(index.cast_unsigned()).transpose()
                }
            }
            _ => Self::Vec(self.to_vec()?).at(index),
        }
    }

    #[must_use]
    pub fn has(self, t: &Self) -> bool {
        match self {
            Self::Set(ts) => ts.contains(t),
            Self::Map(kvs) => kvs.contains_key(t),
            Self::Vec(ts) => ts.contains(t),
            Self::Seq(mut it) => it.contains(&Ok(t.clone())),
            _ => self.to_vec().is_ok_and(|ts| Self::Vec(ts).has(t)),
        }
    }

    pub fn remove(mut self, t: &Self) -> EuRes<(Option<Self>, Self)> {
        match self {
            Self::Set(ref mut ts) => Ok((Rc::make_mut(ts).remove_full(t).map(|(_, t)| t), self)),
            Self::Map(ref mut kvs) => Ok((
                Rc::make_mut(kvs)
                    .remove_full(t)
                    .map(|(_, k, v)| Self::vec([k, v])),
                self,
            )),
            _ => self.remove_index(t.try_isize()?),
        }
    }

    pub fn remove_index(mut self, index: isize) -> EuRes<(Option<Self>, Self)> {
        let check = |len: usize| {
            if index < 0 {
                len.checked_add_signed(index)
            } else {
                Some(index.cast_unsigned())
            }
        };

        match self {
            Self::Vec(ref mut ts) => Ok((check(ts.len()).map(|i| ts.remove(i)), self)),
            Self::Map(ref mut kvs) => Ok((
                check(kvs.len()).and_then(|i| {
                    Rc::make_mut(kvs)
                        .remove_index(i)
                        .map(|(k, v)| Self::vec([k, v]))
                }),
                self,
            )),
            Self::Set(ref mut ts) => Ok((
                check(ts.len()).and_then(|i| Rc::make_mut(ts).remove_index(i)),
                self,
            )),
            Self::Opt(ref mut o) => Ok((
                (index == 0 || index == -1)
                    .then(|| o.take().map(|t| *t))
                    .flatten(),
                self,
            )),
            Self::Res(_) => Self::opt(self.to_opt()).pop_back(),
            Self::Expr(ref mut ts) => Ok((check(ts.len()).map(|i| ts.remove(i).into()), self)),
            Self::Str(ref mut s) => Ok((
                check(s.len()).map(|i| {
                    let mut r = s.mutate();
                    Self::Char(r.remove(i))
                }),
                self,
            )),
            _ => Self::Vec(self.to_vec()?).remove_index(index),
        }
    }

    pub fn swap_remove(mut self, t: &Self) -> EuRes<(Option<Self>, Self)> {
        match self {
            Self::Set(ref mut ts) => {
                Ok((Rc::make_mut(ts).swap_remove_full(t).map(|(_, t)| t), self))
            }
            Self::Map(ref mut kvs) => Ok((
                Rc::make_mut(kvs)
                    .swap_remove_full(t)
                    .map(|(_, k, v)| Self::vec([k, v])),
                self,
            )),
            _ => self.swap_remove_index(t.try_isize()?),
        }
    }

    pub fn swap_remove_index(mut self, index: isize) -> EuRes<(Option<Self>, Self)> {
        let check = |len: usize| {
            if index < 0 {
                len.checked_add_signed(index)
            } else {
                Some(index.cast_unsigned())
            }
        };

        match self {
            Self::Vec(ref mut ts) => {
                let l = ts.len();
                Ok((
                    check(l).and_then(|i| {
                        ts.make_mut().swap(i, l - 1);
                        ts.pop()
                    }),
                    self,
                ))
            }
            Self::Map(ref mut kvs) => Ok((
                check(kvs.len()).and_then(|i| {
                    Rc::make_mut(kvs)
                        .remove_index(i)
                        .map(|(k, v)| Self::vec([k, v]))
                }),
                self,
            )),
            Self::Set(ref mut ts) => Ok((
                check(ts.len()).and_then(|i| Rc::make_mut(ts).remove_index(i)),
                self,
            )),
            Self::Opt(ref mut o) => Ok((
                (index == 0 || index == -1)
                    .then(|| o.take().map(|t| *t))
                    .flatten(),
                self,
            )),
            Self::Res(_) => Self::opt(self.to_opt()).pop_back(),
            Self::Expr(ref mut ts) => Ok((check(ts.len()).map(|i| ts.remove(i).into()), self)),
            Self::Str(ref mut s) => Ok((
                check(s.len()).map(|i| {
                    let mut r = s.mutate();
                    Self::Char(r.remove(i))
                }),
                self,
            )),
            _ => Self::Vec(self.to_vec()?).remove_index(index),
        }
    }

    pub fn push_back(mut self, t: Self) -> EuRes<Self> {
        match self {
            Self::Seq(it) => Ok(Self::seq(it.chain(iter::once(Ok(t))))),
            Self::Vec(ref mut ts) => {
                ts.push(t);
                Ok(self)
            }
            Self::Map(ref mut kvs) => {
                let (k, v) = t.to_pair()?;
                Rc::make_mut(kvs).insert(k, v);
                Ok(self)
            }
            Self::Set(ref mut ts) => {
                Rc::make_mut(ts).insert(t);
                Ok(self)
            }
            Self::Expr(ref mut ts) => {
                ts.push(EuSyn::Raw(t));
                Ok(self)
            }
            Self::Str(ref mut s) => {
                if let Self::Char(c) = t {
                    s.push(c);
                } else {
                    s.push_str(&t.to_string());
                }
                Ok(self)
            }
            _ => Self::Vec(self.to_vec()?).push_back(t),
        }
    }

    #[inline]
    pub fn push_front(self, t: Self) -> EuRes<Self> {
        self.insert(0, t)
    }

    pub fn insert(mut self, index: isize, mut t: Self) -> EuRes<Self> {
        let a = index.unsigned_abs();
        let check = |len: usize| {
            let hi = len.cast_signed();
            let low = -hi - 1;
            (low <= index && index <= hi)
                .ok_or_else(|| anyhow!("{index} out of bounds [{low}, {hi}]"))
                .map(|()| if index < 0 { len + 1 - a } else { a })
        };

        match self {
            Self::Vec(ref mut ts) => {
                ts.insert(check(ts.len())?, t);
                Ok(self)
            }
            Self::Map(ref mut kvs) => {
                let a = check(kvs.len())?;
                let (k, v) = t.to_pair()?;
                Rc::make_mut(kvs).insert_before(a, k, v);
                Ok(self)
            }
            Self::Set(ref mut ts) => {
                let a = check(ts.len())?;
                Rc::make_mut(ts).insert_before(a, t);
                Ok(self)
            }
            Self::Seq(it) => {
                if index < 0 {
                    Self::Vec(Self::Seq(it).to_vec()?).insert(index, t)
                } else {
                    let mut i = 0;
                    let mut ins = false;
                    Ok(Self::seq(it.batching(move |it| {
                        if !ins && i == a {
                            ins = true;
                            Some(Ok(mem::take(&mut t)))
                        } else {
                            let t = it.next();
                            if t.is_some() {
                                i += 1;
                            }
                            t
                        }
                    })))
                }
            }
            Self::Expr(ref mut ts) => {
                ts.insert(check(ts.len())?, EuSyn::Raw(t));
                Ok(self)
            }
            Self::Str(s) => {
                let a = check(s.len())?;
                let mut res = s.slice(0..a);
                if let Self::Char(c) = t {
                    res.push(c);
                } else {
                    res.push_str(&t.to_string());
                }
                res.push_str(&s.slice(a..));
                Ok(Self::Str(res))
            }
            _ => Self::Vec(self.to_vec()?).insert(index, t),
        }
    }

    pub fn append(self, other: Self) -> EuRes<Self> {
        match (self, other) {
            (Self::Map(a), Self::Map(b)) => {
                let mut a = Rc::unwrap_or_clone(a);
                a.append(&mut Rc::unwrap_or_clone(b));
                Ok(Self::map_(a))
            }
            (Self::Set(a), Self::Set(b)) => {
                let mut a = Rc::unwrap_or_clone(a);
                a.append(&mut Rc::unwrap_or_clone(b));
                Ok(Self::set(a))
            }
            (Self::Char(a), Self::Char(b)) => {
                let mut s = LocalHipStr::with_capacity(2);
                s.push(a);
                s.push(b);
                Ok(Self::Str(s))
            }
            (a, b) if a.is_seq() || b.is_seq() => Ok(Self::seq(a.to_seq().chain(b.to_seq()))),
            (a, b) if a.is_vec() || b.is_vec() => {
                let mut a = a.to_vec()?;
                a.extend(b.to_vec()?);
                Ok(Self::Vec(a))
            }
            (a, b) if a.is_expr() || b.is_expr() => {
                let mut a = a.to_expr()?;
                a.extend((b).to_expr()?);
                Ok(Self::Expr(a))
            }
            (a, b) if a.is_str() || b.is_str() => Ok(Self::str(format!("{a}{b}"))),
            (a, b) => Self::Vec(a.to_vec()?).append(b),
        }
    }

    pub fn pop_back(mut self) -> EuRes<(Option<Self>, Self)> {
        match self {
            Self::Vec(ref mut ts) => Ok((ts.pop(), self)),
            Self::Map(ref mut kvs) => Ok((
                Rc::make_mut(kvs).pop().map(|(k, v)| Self::vec([k, v])),
                self,
            )),
            Self::Set(ref mut ts) => Ok((Rc::make_mut(ts).pop(), self)),
            Self::Opt(ref mut o) => Ok((o.take().map(|t| *t), self)),
            Self::Res(_) => Self::opt(self.to_opt()).pop_back(),
            Self::Expr(ref mut ts) => Ok((ts.pop().map(EuSyn::into), self)),
            Self::Str(ref mut s) => Ok((s.pop().map(Self::Char), self)),
            _ => Self::Vec(self.to_vec()?).pop_back(),
        }
    }

    pub fn pop_front(self) -> EuRes<(Option<Self>, Self)> {
        self.remove_index(0)
    }
}
