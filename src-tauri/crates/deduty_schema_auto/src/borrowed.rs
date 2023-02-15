use std::borrow::Borrow;

use async_std::sync::Arc;


pub type Borrowed<T> = Box<dyn Borrow<T> + Send + Sync>;
pub type BorrowedIterator<T> = Box<dyn Iterator<Item = Borrowed<T>> + Send + Sync>;


/// Struct for containment T under Arc (for origin owner)
pub struct BorrowedItem<T: Send + Sync + ?Sized> {
    owned: Arc<T>
}

impl<'s, T: Send + Sync + ?Sized + 's> BorrowedItem<T> {
    pub fn boxed(owned: Arc<T>) -> Box<dyn Borrow<T> + Send + Sync +'s> {
        Box::new(Self { owned })
    }
}

impl<T: Sync + Send + ?Sized> Borrow<T> for BorrowedItem<T> {
    fn borrow(&self) -> &T {
        self.owned.as_ref()
    }
}
