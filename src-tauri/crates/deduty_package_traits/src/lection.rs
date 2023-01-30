use std::any::Any;

use crate::file::DedutyFileCollection;


pub trait DedutyLectionMeta: Sync + Send {
    fn name(&self) -> String;
    fn order(&self) -> u64;
}

pub trait DedutyLection: Sync + Send {
    fn as_any_ref(&self) -> &(dyn Any + Send + Sync);
    fn as_any_mut(&mut self) -> &mut (dyn Any + Send + Sync);

    fn id(&self) -> String;
    fn meta(&self) -> &dyn DedutyLectionMeta;
    fn files(&self) -> &dyn DedutyFileCollection;
}
