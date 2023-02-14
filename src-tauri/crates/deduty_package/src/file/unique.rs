pub trait UniqueFile: Sync + Send {
    fn id(&self) -> &str;
}
