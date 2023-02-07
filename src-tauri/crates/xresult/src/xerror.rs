use std::error::Error;
use std::fmt::Display;


#[derive(Debug)]
pub struct XError {
    name: String,
    message: String
}

impl XError {
    pub fn new(name: String, message: String) -> Self {
        Self { name, message }
    }
}

impl Error for XError {}
impl Display for XError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_fmt(format_args!("{}: {}", self.name, self.message))
    }
}

impl<K: Display, M: Display> From<(K, M)> for XError
{
    fn from((name, message): (K, M)) -> Self {
        Self {
            name: name.to_string(),
            message: message.to_string()
        }
    }
}

impl<T> From<XError> for crate::XResult<T> {
    fn from(value: XError) -> Self {
        Err(Box::new(value))
    }
}
