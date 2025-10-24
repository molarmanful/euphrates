use std::{
    cmp::Ordering,
    error,
    hash::{
        Hash,
        Hasher,
    },
    sync::Arc,
};

use derive_more::Display;

#[derive(Debug, Display, Clone)]
pub struct EuErr(pub Arc<anyhow::Error>);

impl error::Error for EuErr {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.0.source()
    }
}

impl From<anyhow::Error> for EuErr {
    fn from(err: anyhow::Error) -> Self {
        Self(Arc::new(err))
    }
}

impl PartialEq for EuErr {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for EuErr {}

impl PartialOrd for EuErr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EuErr {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl Hash for EuErr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}
