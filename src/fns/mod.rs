mod core;
pub use core::*;

use crate::env::EuEnv;

pub type EuDef = fn(&mut EuEnv) -> anyhow::Result<()>;
