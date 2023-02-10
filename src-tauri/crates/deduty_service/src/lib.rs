#![feature(trait_upcasting)]

mod add;
mod index;
mod statefull;
mod storage;
mod sub;
mod unique;
mod update;

pub use add::AddService;
pub use index::IndexService;
pub use statefull::StateFullService;
pub use storage::WebStorageService;
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
