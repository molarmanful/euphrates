mod base;

use std::{
    cmp::Ordering,
    sync::Arc,
};

pub use base::*;
use derive_more::{
    Display,
    Error,
};
use dyn_clone::DynClone;

pub type EuIter<'eu, T = EuType<'eu>> = Box<dyn Iterator<Item = T> + 'eu>;

type EuSeq<'eu> = Box<dyn CloneIter<'eu, EuRes<EuType<'eu>>> + 'eu>;

pub trait CloneIter<'t, T>
where
    T: 't,
    Self: Iterator<Item = T> + DynClone,
{
}

impl<'t, T: 't, I> CloneIter<'t, T> for I where Self: Iterator<Item = T> + DynClone {}

dyn_clone::clone_trait_object!(<T> CloneIter<'_, T>);

pub type EuRes<T> = anyhow::Result<T, EuErr>;

#[derive(Debug, Display, Clone, Error)]
pub struct EuErr(pub Arc<anyhow::Error>);

impl From<anyhow::Error> for EuErr {
    fn from(err: anyhow::Error) -> Self {
        Self(Arc::new(err))
    }
}

impl PartialEq for EuErr {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for EuErr {}

impl PartialOrd for EuErr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EuErr {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_string().cmp(&other.to_string())
    }
}
