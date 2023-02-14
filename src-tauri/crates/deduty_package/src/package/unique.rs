
pub trait UniquePackage: Sync + Send {
    fn id(&self) -> &str;
}
