#![feature(trait_upcasting)]
use std::borrow::Borrow;

mod file;
mod lection;
mod package;
mod reader;

#[cfg(feature = "serde")]
pub mod serde;

pub use reader::DedutyFileReader;

pub use file::{
    UniqueFile,
    MetaFile,
    ReadFile,
    SerdeFile,
    DedutyFile,
};

pub use lection::{
    UniqueLection,
    MetaLection,
    PeekLection,
    ReadLection,
    SerdeLection,
    DedutyLection
};

pub use package::{
    UniquePackage,
    MetaPackage,
    PeekPackage,
    ReadPackage,
    SerdePackage,
    DedutyPackage,
};


pub type Borrowed<T> = Box<dyn Borrow<T> + Send + Sync>;
pub type BorrowedIterator<T> = Box<dyn Iterator<Item = Borrowed<T>> + Send + Sync>;
