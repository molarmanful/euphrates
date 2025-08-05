pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[inline]
pub fn swap_errors<T, E0, E1>(r: Result<Result<T, E0>, E1>) -> Result<Result<T, E1>, E0> {
    match r {
        Ok(Ok(t)) => Ok(Ok(t)),
        Ok(Err(e)) => Err(e),
        Err(e) => Ok(Err(e)),
    }
}
