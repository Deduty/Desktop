use async_trait::async_trait;
use xresult::XResult;


pub trait UniquePackage: Sync + Send {
    fn id(&self) -> &str;
}


pub trait MetaPackage: Sync + Send {
    /// Json acceptable string
    fn meta(&self) -> Option<&str>;

    /// Indicates how much space the package occupies on the machine:
    /// - Use [`None`] if package size **can't be** known without it's loading.
    /// - Use [`Some(0)`] when package is not exist on the machine.
    /// - Otherwise [`Some(usize)`] where [`usize`] is the package `bytes` size on the machine.
    /// 
    /// Note: This is not same as sum of lection files sizes.
    ///       This method indicates only lection size.
    fn size(&self) -> Option<usize>;
}


/// This cheap trait will be used each time,
///     when frontend needs to obtain lection id
///     or get whole package size must be cached if possible
#[async_trait]
pub trait PeekPackage: Sync + Send {
    async fn lections(&self) -> XResult<Box<dyn Iterator<Item = &dyn crate::SerdeLection> + Send>>;
}


/// This expensive trait will be used when application needs
///     to read some or all files when frontend needs to obtain
///     files ids must be cached if possible
#[async_trait]
pub trait ReadPackage: Sync + Send {
    async fn lection(&self, id: &str) -> XResult<Option<&dyn crate::DedutyLection>>;
    async fn lections(&self) -> XResult<Box<dyn Iterator<Item = &dyn crate::DedutyLection> + Send>>;
}


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
