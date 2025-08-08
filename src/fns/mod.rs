mod core;
pub use core::*;

use crate::{
    env::EuEnv,
    types::EuRes,
};

pub type EuDef = fn(&mut EuEnv) -> EuRes<()>;
