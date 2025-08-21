mod iter_ext;
use std::{
    any::Any,
    panic::{
        self,
        UnwindSafe,
    },
};

pub use iter_ext::*;

#[inline]
pub fn swap_errors<T, E0, E1>(r: Result<Result<T, E0>, E1>) -> Result<Result<T, E1>, E0> {
    match r {
        Ok(Ok(t)) => Ok(Ok(t)),
        Ok(Err(e)) => Err(e),
        Err(e) => Ok(Err(e)),
    }
}

pub fn unpanic<F, R>(f: F) -> Result<R, Box<dyn Any + Send + 'static>>
where
    F: FnOnce() -> R + UnwindSafe,
{
    let prev = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let res = panic::catch_unwind(f);
    panic::set_hook(prev);
    res
}
