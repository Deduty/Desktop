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
