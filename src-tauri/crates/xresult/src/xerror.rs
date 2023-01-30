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

// AUTO FROM IMPLS

impl From<(String, String)> for XError {
    fn from((name, message): (String, String)) -> Self {
        Self { name, message }
    }
}

impl From<(&String, &String)> for XError {
    fn from((name, message): (&String, &String)) -> Self {
        (name.clone(), message.clone()).into()
    }
}

impl From<(&str, String)> for XError {
    fn from((name, message): (&str, String)) -> Self {
        (name.to_string(), message).into()
    }
}

impl From<(&str, &String)> for XError {
    fn from((name, message): (&str, &String)) -> Self {
        (name.to_string(), message.clone()).into()
    }
}

impl From<(&str, &str)> for XError {
    fn from((name, message): (&str, &str)) -> Self {
        (name.clone(), message.clone()).into()
    }
}

impl<T> Into<crate::XResult<T>> for XError {
    fn into(self) -> crate::XResult<T> {
        Err(Box::new(self))
    }
}
