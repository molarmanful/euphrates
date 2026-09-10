use crate::{
    fns::{
        EuDef,
        macros::f_2_to_1,
    },
    types::EuType,
};

pub const CMP: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1")?;
        let a0 = env.arg("a0")?;
        env.push(EuType::i32(a0.cmp(&a1) as i32));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

macro_rules! cmp_binop {
    ($op:tt) => {
        |env| {
            let a1 = env.arg("a1")?;
            let a0 = env.arg("a0")?;
            env.push(EuType::Bool(a0 $op a1));
            Ok(())
        }
    };
}

pub const EQ: EuDef = EuDef {
    def: cmp_binop!(==),
    sigs: &[],
    doc: "",
};

pub const NE: EuDef = EuDef {
    def: cmp_binop!(!=),
    sigs: &[],
    doc: "",
};

pub const LT: EuDef = EuDef {
    def: cmp_binop!(<),
    sigs: &[],
    doc: "",
};

pub const LE: EuDef = EuDef {
    def: cmp_binop!(<=),
    sigs: &[],
    doc: "",
};

pub const GT: EuDef = EuDef {
    def: cmp_binop!(>),
    sigs: &[],
    doc: "",
};

pub const GE: EuDef = EuDef {
    def: cmp_binop!(>=),
    sigs: &[],
    doc: "",
};

pub const LOOSE_CMP: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1")?;
        let a0 = env.arg("a0")?;
        env.push(EuType::i32(a0.loose_cmp(&a1) as i32));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

macro_rules! loose_cmp_binop {
    ($check:ident) => {
        |env| {
            let a1 = env.arg("a1")?;
            let a0 = env.arg("a0")?;
            env.push(EuType::Bool(a0.loose_cmp(&a1).$check()));
            Ok(())
        }
    };
}

pub const LOOSE_EQ: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1")?;
        let a0 = env.arg("a0")?;
        env.push(EuType::Bool(a0.loose_eq(&a1)));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const LOOSE_NE: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1")?;
        let a0 = env.arg("a0")?;
        env.push(EuType::Bool(!a0.loose_eq(&a1)));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const LOOSE_LT: EuDef = EuDef {
    def: loose_cmp_binop!(is_lt),
    sigs: &[],
    doc: "",
};

pub const LOOSE_LE: EuDef = EuDef {
    def: loose_cmp_binop!(is_le),
    sigs: &[],
    doc: "",
};

pub const LOOSE_GT: EuDef = EuDef {
    def: loose_cmp_binop!(is_gt),
    sigs: &[],
    doc: "",
};

pub const LOOSE_GE: EuDef = EuDef {
    def: loose_cmp_binop!(is_ge),
    sigs: &[],
    doc: "",
};

pub const MIN: EuDef = EuDef {
    def: f_2_to_1!(min),
    sigs: &[],
    doc: "",
};

pub const MAX: EuDef = EuDef {
    def: f_2_to_1!(max),
    sigs: &[],
    doc: "",
};

pub const LOOSE_MIN: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1")?;
        let a0 = env.arg("a0")?;
        env.push(if a0.loose_cmp(&a1).is_le() { a0 } else { a1 });
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const LOOSE_MAX: EuDef = EuDef {
    def: |env| {
        let a1 = env.arg("a1")?;
        let a0 = env.arg("a0")?;
        env.push(if a0.loose_cmp(&a1).is_ge() { a0 } else { a1 });
        Ok(())
    },
    sigs: &[],
    doc: "",
};
