use derive_more::{
    Debug,
    From,
    IntoIterator,
};

use super::EuType;

#[derive(Debug, PartialEq, Clone, From, IntoIterator, Default)]
#[from(forward)]
#[into_iterator(owned, ref, ref_mut)]
pub struct EuVec<'eu>(pub Vec<EuType<'eu>>);

impl EuVec<'_> {
    pub fn iflip(&self, i: usize) -> usize {
        self.0.len() - i - 1
    }
}
