use crate::file::DedutyFileCollection;
use crate::lection::DedutyLection;


pub trait DedutyPackageMeta: Sync + Send {
    fn name(&self) -> String;
    fn version(&self) -> String;
    fn language(&self) -> String;
    fn tags(&self) -> Vec<String>;
}

pub trait DedutyPackage: Sync + Send {
    fn id(&self) -> String;
    fn meta(&self) -> &dyn DedutyPackageMeta;
    fn files(&self) -> &dyn DedutyFileCollection;
    fn lections(&self) -> Vec<&dyn DedutyLection>;
}
