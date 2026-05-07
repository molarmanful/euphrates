use rand::RngExt;

use crate::{
    fns::EuDef,
    types::EuType,
};

#[crabtime::function]
fn rand(name: String) {
    let f = name.to_lowercase();
    crabtime::output! {
        pub const RAND_{{name}}: EuDef = |env| {
            let n: {{f}} = env.ctx.rng.borrow_mut().random();
            env.push(EuType::{{f}}(n));
            Ok(())
        };
    }
}

rand!(I32);
rand!(I64);
rand!(F64);
