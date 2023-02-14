mod meta;
mod peek;
mod read;
mod unique;

pub use meta::MetaLection;
pub use peek::PeekLection;
pub use read::ReadLection;
pub use unique::UniqueLection;


pub trait SerdeLection:
    UniqueLection +
    MetaLection +
    PeekLection +

    Send + Sync
{}

impl<T> SerdeLection for T
    where T:
        UniqueLection +
        MetaLection +
        PeekLection +

        Send + Sync
{}


pub trait DedutyLection:
    SerdeLection +

    UniqueLection +
    MetaLection +
    PeekLection +
    ReadLection +

    Send + Sync
{}

impl<T> DedutyLection for T
    where T:
        SerdeLection +

        UniqueLection +
        MetaLection +
        PeekLection +
        ReadLection +
        
        Send + Sync
{}
