mod add;
mod index;
mod statefull;
mod sub;
mod unique;
mod update;

pub use add::AddService;
pub use index::IndexService;
pub use statefull::StateFullService;
pub use sub::SubService;
pub use unique::UniqueService;
pub use update::UpdateService;


/// ### Important
/// Type that implements this trait must have internal mutability
///
// TODO: Use unique service and try get it as dyn *Service
pub trait Service:
    UniqueService +
    StateFullService +
    IndexService +
    AddService +
    UpdateService +
    SubService +

    Send + Sync
{}
