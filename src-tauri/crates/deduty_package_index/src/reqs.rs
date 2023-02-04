use std::error::Error;
use std::fmt::Display;

use xresult::{ XError, XResult };


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrontEndRequirement {
    DirectoryPath
}

impl Display for FrontEndRequirement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrontEndRequirement::DirectoryPath => fmt.write_str("DirectoryPath"),
        }
    }
}

impl TryFrom<&String> for FrontEndRequirement {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(value: &String) -> XResult<Self> {
        match value.as_str() {
            "DirectoryPath" => Ok(FrontEndRequirement::DirectoryPath),
            other => Err(XError::from(("FrontEnd Requirement parsing error", format!("Requirement `{other}` is not allowed"))).into())
        }
    }
}


#[derive(Debug, Clone)]
pub struct FrontEndSerialization {
    kind: FrontEndRequirement,
    value: String
}

impl FrontEndSerialization {
    pub fn new(kind: FrontEndRequirement, value: String) -> Self {
        Self { kind, value }
    }

    pub fn kind(&self) -> &FrontEndRequirement {
        &self.kind
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn unwrap(&self, kind: &FrontEndRequirement) -> XResult<&String> {
        match self.kind == *kind {
            true => Ok(&self.value),
            false => Err(XError::from(("FrontEndRequirement unwrap error", format!("Requirement kind is `{}` but `{kind}` expected", self.kind))).into())
        }
    }
}

impl TryFrom<(String, FrontEndRequirement)> for FrontEndSerialization {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from((value, kind): (String, FrontEndRequirement)) -> XResult<Self> {
        Ok(FrontEndSerialization::new(kind, value))
    }
}
