use async_trait::async_trait;
use xresult::XResult;


pub trait UniqueLection: Sync + Send {
    fn id(&self) -> &String;
}


pub trait MetaLection: Sync + Send {
    /// Json acceptable string
    fn meta(&self) -> Option<&String>;

    /// Indicates how much space the lection occupies on the machine:
    /// - Use [`None`] if lection size **can't be** known without it's loading.
    /// - Use [`Some(0)`] when lection is not exist on the machine.
    /// - Otherwise [`Some(usize)`] where [`usize`] is the lection `bytes` size on the machine.
    /// 
    /// Note: This is not same as sum of lection files sizes.
    ///       This method indicates only lection size.
    fn size(&self) -> Option<usize>;
}


/// This trait is used when application needs to serialize or calculate file size.
/// Must be **cached** (if possible) and **can be used very often**.
#[async_trait]
pub trait PeekLection: Sync + Send {
    async fn files(&self) -> XResult<Box<dyn Iterator<Item = &dyn crate::SerdeFile> + Send>>;
}


/// This trait is used when application needs to read file or make expensive operation.
#[async_trait]
pub trait ReadLection: Sync + Send {
    async fn file(&self, id: &str) -> XResult<Option<&dyn crate::DedutyFile>>;
    async fn files(&self) -> XResult<Box<dyn Iterator<Item = &dyn crate::DedutyFile> + Send>>;
}


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
