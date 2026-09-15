use rand::RngExt;

use crate::{
    fns::EuDef,
    types::EuType,
};

macro_rules! rand {
    ($t:ident) => {
        |env| {
            let n: $t = env.rng().borrow_mut().random();
            env.push(EuType::$t(n));
            Ok(())
        }
    };
}

pub const RAND_I32: EuDef = EuDef {
    def: rand!(i32),
    sigs: &[],
    doc: "",
};

pub const RAND_I64: EuDef = EuDef {
    def: rand!(i64),
    sigs: &[],
    doc: "",
};

pub const RAND_F64: EuDef = EuDef {
    def: rand!(f64),
    sigs: &[],
    doc: "",
};
