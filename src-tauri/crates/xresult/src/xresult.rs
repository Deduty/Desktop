use std::error::Error;

pub type XResult<T> = Result<T, Box<dyn Error + Send + Sync>>;
