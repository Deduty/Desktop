use std::any::Any;

use crate::file::DedutyFileCollection;
use crate::lection::DedutyLection;


pub trait DedutyPackageMeta: Sync + Send {
    fn name(&self) -> String;
    fn version(&self) -> String;
    fn language(&self) -> String;
    fn tags(&self) -> Vec<String>;
}

pub trait DedutyPackage: Any + Sync + Send {
    fn as_any_ref(&self) -> &(dyn Any + Send + Sync);
    fn as_any_mut(&mut self) -> &mut (dyn Any + Send + Sync);

    fn id(&self) -> String;
    fn meta(&self) -> &dyn DedutyPackageMeta;
    fn files(&self) -> &dyn DedutyFileCollection;
    fn lections(&self) -> Vec<&dyn DedutyLection>;
}
