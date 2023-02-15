use xresult::{ XError, XResult };


/// Common error constructor
pub fn error<M: std::fmt::Display>(message: M) -> Box<dyn std::error::Error + Send + Sync> {
    XError::from(("Auto service error", message)).into()
}

/// Common error as result
pub fn error_err<T, M: std::fmt::Display>(message: M) -> XResult<T> {
    XError::from(("Auto service error", message)).into()
}
