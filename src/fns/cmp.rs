use super::EuDef;
use crate::types::EuType;

pub const CMP: EuDef = |env| {
    env.check_nargs(2)?;
    let a1 = env.stack.pop().unwrap();
    let a0 = env.stack.pop().unwrap();
    env.push(EuType::i32(a0.cmp(&a1) as i32));
    Ok(())
};

#[crabtime::function]
fn gen_fn_cmp_binops() {
    for (name, op) in [
        ("EQ", "=="),
        ("NE", "!="),
        ("LT", "<"),
        ("LE", "<="),
        ("GT", ">"),
        ("GE", ">="),
    ] {
        crabtime::output! {
            pub const {{name}}: EuDef = |env| {
                env.check_nargs(2)?;
                let a1 = env.stack.pop().unwrap();
                let a0 = env.stack.pop().unwrap();
                env.push(EuType::Bool(a0 {{op}} a1));
                Ok(())
            };
        }
    }
}

gen_fn_cmp_binops!();
