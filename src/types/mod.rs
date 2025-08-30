mod base;
mod err;
mod iter;
mod num;
mod ord;

pub use base::*;
use dyn_clone::DynClone;
pub use err::*;

pub type EuIter<'eu, T = EuType<'eu>> = Box<dyn Iterator<Item = T> + 'eu>;

type EuSeq<'eu> = Box<dyn CloneIter<'eu, EuRes<EuType<'eu>>> + 'eu>;

pub trait EuSeqImpl<'eu> = Iterator<Item = EuRes<EuType<'eu>>> + Clone;

pub trait CloneIter<'t, T>
where
    T: 't,
    Self: Iterator<Item = T> + DynClone,
{
}

impl<'t, T: 't, I> CloneIter<'t, T> for I where Self: Iterator<Item = T> + DynClone {}

dyn_clone::clone_trait_object!(<T> CloneIter<'_, T>);

pub type EuRes<T> = anyhow::Result<T, EuErr>;
