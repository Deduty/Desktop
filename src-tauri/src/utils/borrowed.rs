use std::sync::Arc;
use std::borrow::Borrow;

/// Type alias
pub type Borrowed<'s, T> = Box<dyn Borrow<T> + Send + Sync + 's>;

/// Type alias
pub type BorrowedIterator<'s, T> = Box<dyn Iterator<Item = Borrowed<'s, T>> + Send + Sync>;


/// Struct for containment T under Arc (for origin owner)
pub struct BorrowedItem<T: Send + Sync + ?Sized> {
    owned: Arc<T>
}

impl<'s, T: Send + Sync + ?Sized + 's> BorrowedItem<T> {
    pub fn boxed(owned: Arc<T>) -> Borrowed<'s, T> {
        Box::new(Self { owned })
    }
}

impl<T: Sync + Send + ?Sized> Borrow<T> for BorrowedItem<T> {
    fn borrow(&self) -> &T {
        self.owned.as_ref()
    }
}
