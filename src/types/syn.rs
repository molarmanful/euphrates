use derive_more::{
    Debug,
    Display,
    IsVariant,
};
use ecow::EcoVec;
use itertools::Itertools;

use super::EuType;

#[derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Clone, IsVariant)]
#[display("{_0}")]
pub enum EuSyn<'eu> {
    #[debug("({})#vec", _0.iter().map(|t| format!("{t:?}")).join(" "))]
    #[display("{}", _0.iter().join(" "))]
    Vec(EcoVec<Self>),
    #[debug("{_0:?}")]
    Raw(EuType<'eu>),
}

impl<'eu> From<EuType<'eu>> for EuSyn<'eu> {
    fn from(value: EuType<'eu>) -> Self {
        Self::Raw(value)
    }
}

impl<'eu> From<EuSyn<'eu>> for EuType<'eu> {
    fn from(value: EuSyn<'eu>) -> Self {
        match value {
            EuSyn::Vec(ts) => EuType::Vec(ts.into_iter().map(Self::from).collect()),
            EuSyn::Raw(t) => t,
        }
    }
}
