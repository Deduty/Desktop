pub trait UniqueLection: Sync + Send {
    fn id(&self) -> &str;
}
