use std::rc::Rc;

use crate::types::EuType;

pub enum ManyIntoIter<'eu> {
    Vec(ecow::vec::IntoIter<EuType<'eu>>),
    Map(ordermap::map::IntoValues<EuType<'eu>, EuType<'eu>>),
    Set(ordermap::set::IntoIter<EuType<'eu>>),
}

impl<'eu> Iterator for ManyIntoIter<'eu> {
    type Item = EuType<'eu>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Vec(it) => it.next(),
            Self::Map(it) => it.next(),
            Self::Set(it) => it.next(),
        }
    }
}

impl<'eu> IntoIterator for EuType<'eu> {
    type IntoIter = ManyIntoIter<'eu>;
    type Item = EuType<'eu>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Vec(ts) => Self::IntoIter::Vec(ts.into_iter()),
            Self::Map(kvs) => Self::IntoIter::Map(Rc::unwrap_or_clone(kvs).into_values()),
            Self::Set(ts) => Self::IntoIter::Set(Rc::unwrap_or_clone(ts).into_iter()),
            _ => unreachable!(),
        }
    }
}
