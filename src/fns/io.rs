use std::io;

use anyhow::anyhow;

use crate::{
    fns::EuDef,
    types::EuType,
};

pub const READ: EuDef = EuDef {
    def: |env| {
        env.push(EuType::res_str(
            io::read_to_string(io::stdin())
                .map(EuType::str)
                .map_err(|e| anyhow!(e).into()),
        ));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const READLN: EuDef = EuDef {
    def: |env| {
        let mut res = String::new();
        env.push(EuType::res_str(
            io::stdin()
                .read_line(&mut res)
                .map(|_| EuType::str(res))
                .map_err(|e| anyhow!(e).into()),
        ));
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const PRINT: EuDef = EuDef {
    def: |env| {
        print!("{}", env.arg("a0")?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};

pub const PRINTLN: EuDef = EuDef {
    def: |env| {
        println!("{}", env.arg("a0")?);
        Ok(())
    },
    sigs: &[],
    doc: "",
};
