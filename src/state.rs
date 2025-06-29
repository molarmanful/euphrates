use std::collections::HashMap;

use crate::types::{
    EuType,
    EuVec,
};

struct State<'eu> {
    stack: EuVec<'eu>,
    scope: HashMap<String, EuType<'eu>>,
}

impl<'eu> State<'eu> {
    pub fn new() -> Self {
        todo!()
    }
}
