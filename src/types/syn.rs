use derive_more::{
    Debug,
    Display,
    IsVariant,
};
use ecow::EcoVec;
use hipstr::LocalHipStr;
use itertools::Itertools;

use crate::types::EuType;

#[derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, IsVariant)]
#[display("{_0}")]
pub enum EuSyn<'eu> {
    #[debug("{_0:?}")]
    Raw(EuType<'eu>),
    #[debug("${_0}")]
    Var(LocalHipStr<'eu>),
    #[debug("\\{_0}")]
    Move(LocalHipStr<'eu>),
    #[debug("@Vec({})", _0.iter().map(|t| format!("{t:?}")).join(" "))]
    #[display("{}", _0.iter().join(" "))]
    Vec(EcoVec<Self>),
    #[debug("@Map({})", _0.iter().map(|t| format!("{t:?}")).join(" "))]
    #[display("{}", _0.iter().join(" "))]
    Map(EcoVec<Self>),
}

impl<'eu> From<EuType<'eu>> for EuSyn<'eu> {
    fn from(value: EuType<'eu>) -> Self {
        Self::Raw(value)
    }
}

impl<'eu> From<EuSyn<'eu>> for EuType<'eu> {
    fn from(value: EuSyn<'eu>) -> Self {
        match value {
            EuSyn::Vec(ts) | EuSyn::Map(ts) => Self::Expr(ts),
            EuSyn::Var(w) | EuSyn::Move(w) => Self::word(w),
            EuSyn::Raw(t) => t,
        }
    }
}
