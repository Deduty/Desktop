mod meta;
mod read;
mod unique;

pub use meta::MetaFile;
pub use read::ReadFile;
pub use unique::UniqueFile;


pub trait SerdeFile:
    UniqueFile +
    MetaFile +
    
    Send + Sync
{}

impl<T> SerdeFile for T
    where T:
        UniqueFile +
        MetaFile +
        
        Send + Sync
{}


pub trait DedutyFile:
    SerdeFile +

    UniqueFile +
    MetaFile +
    ReadFile +

    Sync + Send
{}

impl<T> DedutyFile for T
    where T:
        SerdeFile +

        UniqueFile +
        MetaFile +
        ReadFile +

        Sync + Send
{}
