use std::collections::HashMap;

use crate::types::{
    EuFn,
    EuType,
    EuVec,
};

struct State<'eu> {
    stack: EuVec<'eu>,
    ast: EuFn<'eu>,
    scope: HashMap<String, EuType<'eu>>,
}

impl State<'_> {
    pub fn new() -> Self {
        todo!()
    }
}
