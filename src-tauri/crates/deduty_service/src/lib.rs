#![feature(trait_upcasting)]
use std::borrow::Borrow;

mod add;
mod index;
mod statefull;
mod web_storage;
mod sub;
mod unique;
mod update;

pub use add::AddService;
pub use index::IndexService;
pub use statefull::StateFullService;
pub use web_storage::WebStorageService;
pub use sub::SubService;
pub use unique::UniqueService;
pub use update::UpdateService;


/// ### Important
/// Type that implements this trait must have internal mutability
pub trait Service:
    UniqueService +
    WebStorageService +
    StateFullService +
    IndexService +
    AddService +
    UpdateService +
    SubService +

    Send + Sync
{}

impl<T> Service for T
    where T:
        UniqueService +
        WebStorageService +
        StateFullService +
        IndexService +
        AddService +
        UpdateService +
        SubService +

        Send + Sync + ?Sized
{}

pub type Borrowed<T> = Box<dyn Borrow<T> + Send + Sync>;
pub type BorrowedIterator<T> = Box<dyn Iterator<Item = Borrowed<T>> + Send + Sync>;
