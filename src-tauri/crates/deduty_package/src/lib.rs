#![feature(trait_upcasting)]

mod file;
mod lection;
mod package;

#[cfg(feature = "serde")]
pub mod serde;

pub use file::{
    UniqueFile,
    MetaFile,
    ReadFile,
    SerdeFile,
    DedutyFile,

    DedutyFileReader
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
