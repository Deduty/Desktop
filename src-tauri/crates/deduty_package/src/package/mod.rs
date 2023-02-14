mod meta;
mod peek;
mod read;
mod unique;

pub use meta::MetaPackage;
pub use peek::PeekPackage;
pub use read::ReadPackage;
pub use unique::UniquePackage;


pub trait SerdePackage:
    UniquePackage +
    MetaPackage +
    PeekPackage +

    Send + Sync
{}

impl<T> SerdePackage for T
    where T:
        UniquePackage +
        MetaPackage +
        PeekPackage +

        Send + Sync + ?Sized
{}


pub trait DedutyPackage:
    SerdePackage +

    UniquePackage +
    MetaPackage +
    PeekPackage +
    ReadPackage +

    Send + Sync
{}

impl<T> DedutyPackage for T
    where T:
        SerdePackage +

        UniquePackage +
        MetaPackage +
        PeekPackage +
        ReadPackage +
        
        Send + Sync + ?Sized
{}
