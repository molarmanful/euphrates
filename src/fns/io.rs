use std::io;

use anyhow::anyhow;

use super::EuDef;
use crate::types::EuType;

pub const READ: EuDef = |env| {
    env.push(EuType::res_str(
        io::read_to_string(io::stdin())
            .map(EuType::str)
            .map_err(|e| anyhow!(e).into()),
    ));
    Ok(())
};

pub const READLN: EuDef = |env| {
    let mut res = String::new();
    env.push(EuType::res_str(
        io::stdin()
            .read_line(&mut res)
            .map(|_| EuType::str(res))
            .map_err(|e| anyhow!(e).into()),
    ));
    Ok(())
};

pub const PRINT: EuDef = |env| {
    print!("{}", env.pop()?);
    Ok(())
};

pub const PRINTLN: EuDef = |env| {
    println!("{}", env.pop()?);
    Ok(())
};
