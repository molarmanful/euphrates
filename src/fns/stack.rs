use std::mem;

use crate::{
    env::EuEnv,
    fns::EuDef,
    types::EuType,
};

pub const STACK: EuDef = EuDef {
    def: |env| {
        env.push(EuType::Vec(env.stack.clone()));
        Ok(())
    },
    sigs: &["a* -- a* Vec(a*)"],
    doc: "",
};

pub const DUP: EuDef = EuDef {
    def: |env| {
        env.push(env.last()?.clone());
        Ok(())
    },
    sigs: &["a -- a a"],
    doc: "",
};

pub const DUPD: EuDef = EuDef {
    def: |env| {
        env.check_nargs(2)?;
        env.stack.insert(
            env.iflip(1).unwrap(),
            env.stack[env.iflip(1).unwrap()].clone(),
        );
        Ok(())
    },
    sigs: &["a b -- a a b"],
    doc: "",
};

pub const OVER: EuDef = EuDef {
    def: |env| {
        env.check_nargs(2)?;
        env.push(env.stack[env.iflip(1).unwrap()].clone());
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const DDUP: EuDef = EuDef {
    def: |env| {
        env.check_nargs(2)?;
        env.push(env.stack[env.iflip(1).unwrap()].clone());
        env.push(env.stack[env.iflip(1).unwrap()].clone());
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const EDUP: EuDef = EuDef {
    def: |env| {
        env.check_nargs(3)?;
        for _ in 0..3 {
            env.push(env.stack[env.iflip(2).unwrap()].clone());
        }
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const PICK: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?.try_isize()?;
        env.push(env.stack[env.iflip(a0)?].clone());
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const POP: EuDef = EuDef {
    def: |env| {
        env.pop()?;
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const CLR: EuDef = EuDef {
    def: |env| {
        env.stack.clear();
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const NIP: EuDef = EuDef {
    def: |env| {
        env.check_nargs(2)?;
        env.stack.remove(env.iflip(1).unwrap());
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const PPOP: EuDef = EuDef {
    def: |env| {
        env.check_nargs(2)?;
        env.stack.truncate(env.iflip(1).unwrap());
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const QPOP: EuDef = EuDef {
    def: |env| {
        env.check_nargs(3)?;
        env.stack.truncate(env.iflip(2).unwrap());
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const NIX: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?.try_isize()?;
        env.stack.remove(env.iflip(a0)?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const SWAP: EuDef = EuDef {
    def: |env| {
        env.check_nargs(2)?;
        let a = env.iflip(0).unwrap();
        env.stack.make_mut().swap(a, a - 1);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const REV: EuDef = EuDef {
    def: |env| {
        env.stack.make_mut().reverse();
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const SWAPD: EuDef = EuDef {
    def: |env| {
        env.check_nargs(3)?;
        let a = env.iflip(1).unwrap();
        env.stack.make_mut().swap(a, a - 1);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const TUCK: EuDef = EuDef {
    def: |env| {
        env.check_nargs(2)?;
        env.stack
            .insert(env.iflip(1).unwrap(), env.stack.last().unwrap().clone());
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const TRADE: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?.try_isize()?;
        let i = env.iflip(a0)?;
        let j = env.iflip(0).unwrap();
        env.stack.make_mut().swap(i, j);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const ROT: EuDef = EuDef {
    def: |env| {
        env.check_nargs(3)?;
        let a0 = env.stack.remove(env.iflip(2).unwrap());
        env.push(a0);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const UNROT: EuDef = EuDef {
    def: |env| {
        env.check_nargs(3)?;
        let a0 = env.arg("a0")?;
        env.stack.insert(env.iflip(1).unwrap(), a0);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const ROLL: EuDef = EuDef {
    def: |env| {
        let a0 = env.arg("a0")?.try_isize()?;
        let t = env.stack.remove(env.iflip(a0)?);
        env.push(t);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const UNROLL: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1")?.try_isize()?;
        let i = env.iflip(a1)?;
        let a0 = env.arg("a0")?;
        env.stack.insert(i, a0);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const WRAP: EuDef = EuDef {
    def: |env| {
        let a0 = EuType::Vec(mem::take(&mut env.stack));
        env.stack.push(a0);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const UNWRAP: EuDef = EuDef {
    def: |env| {
        let a0 = env.pop()?.to_vec()?;
        env.stack.extend(a0);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const USURP: EuDef = EuDef {
    def: |env| {
        env.stack = env.pop()?.to_vec()?;
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const SUB_STACK: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (eval)")?.to_expr()?;
        let a0 = env.arg("a0")?.to_vec()?;
        env.push(EuType::Vec(EuEnv::apply(a1, &a0, env.cx())?.stack));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const DIP: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1 (eval)")?.to_expr()?;
        let a0 = env.arg("a0")?;
        env.eval_eager(a1)?;
        env.push(a0);
        Ok(())
    },
    sigs: &[],
    doc: "",
};
