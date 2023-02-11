use async_trait::async_trait;
use xresult::{ XReason, XResult };


#[async_trait]
pub trait DedutyFileReader: Sync + Send {
    fn closed(&self) -> bool;

    async fn close(&self) -> XReason;
    async fn next(&self, chunk: usize) -> XResult<Option<Vec<u8>>>;
}


pub trait UniqueFile: Sync + Send {
    fn id(&self) -> &str;
}


pub trait MetaFile: Sync + Send {
    fn ext(&self) -> &str;

    /// Json acceptable string
    fn meta(&self) -> Option<&str>;

    /// Indicates how much space the file occupies on the machine:
    /// - Use [`None`] if file size **can't be** known without it's loading.
    /// - Use [`Some(0)`] when file is not exist on the machine.
    /// - Otherwise use [`Some(usize)`] where [`usize`] is the file `bytes` size on the machine.
    /// 
    /// Note: Use internal mutability if this changes.
    fn size(&self) -> Option<usize>;
}


#[async_trait]
pub trait ReadFile: Sync + Send {
    async fn reader(&self) -> XResult<Box<dyn DedutyFileReader>>;
}


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
