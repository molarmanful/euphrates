use derive_more::{
    Debug,
    Display,
    IsVariant,
};
use ecow::{
    EcoVec,
    eco_vec,
};
use hipstr::LocalHipStr;
use itertools::Itertools;

use crate::types::{
    EuBind,
    EuType,
};

#[derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, IsVariant)]
pub enum EuSyn<'eu> {
    #[debug("{_0:?}")]
    Raw(EuType<'eu>),
    #[debug("${_0}")]
    Var(LocalHipStr<'eu>),
    #[debug("\\{_0}")]
    Move(LocalHipStr<'eu>),
    #[debug("$Vec({})", _0.iter().map(|t| format!("{t:?}")).join(" "))]
    #[display("{}", _0.iter().join(" "))]
    Vec(EcoVec<Self>),
    #[debug("$Map({})", _0.iter().map(|t| format!("{t:?}")).join(" "))]
    #[display("{}", _0.iter().join(" "))]
    Map(EcoVec<Self>),
    #[debug("\\[{}]", _0.iter().map(|t| format!("{t:?}")).join(" "))]
    #[display("{}", _0.iter().join(" "))]
    Bind(EcoVec<EuBind<'eu>>),
}

impl<'eu> From<EuType<'eu>> for EuSyn<'eu> {
    fn from(value: EuType<'eu>) -> Self {
        Self::Raw(value)
    }
}

impl<'eu> From<EuBind<'eu>> for EuSyn<'eu> {
    fn from(value: EuBind<'eu>) -> Self {
        Self::Bind(eco_vec![value])
    }
}
