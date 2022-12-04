use std::error::Error;
use std::fmt::Display;


pub type XResult<T> = Result<T, Box<dyn Error + Send + Sync>>;


#[derive(Debug)]
pub(crate) struct PremierError {
    message: String
}

impl PremierError {
  pub(crate) fn new(message: String) -> Self {
        Self { message }
    }
}

impl Error for PremierError {}
impl Display for PremierError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_fmt(format_args!("Internal error: {}", self.message))
    }
}
