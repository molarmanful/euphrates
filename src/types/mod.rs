mod base;
mod err;
mod iter;
mod num;
mod ord;
mod syn;

pub use std::hash::{
    Hash,
    Hasher,
};

pub use base::*;
use dyn_clone::DynClone;
pub use err::*;
pub use syn::*;

pub type EuIter<'eu, T = EuSyn<'eu>> = Box<dyn Iterator<Item = T> + 'eu>;

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

impl<T: Hash> Hash for dyn CloneIter<'_, T> + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut n: usize = 0;
        for t in dyn_clone::clone_box(self) {
            t.hash(state);
            n += 1;
        }
        n.hash(state);
    }
}

pub type EuRes<T> = anyhow::Result<T, EuErr>;
