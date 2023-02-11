pub trait UniqueService: Send + Sync {
    fn id(&self) -> &str;
}
