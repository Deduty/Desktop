use uuid::Uuid;

use crate::file::traits::DedutyFileCollection;
use crate::lection::traits::DedutyLection;


pub trait DedutyPackageMeta: Sync + Send {
    fn name(&self) -> &String;
    fn version(&self) -> &String;
    fn language(&self) -> &String;
    fn tags(&self) -> &Vec<String>;
}

pub trait DedutyPackage: Sync + Send {
    fn id(&self) -> &Uuid;
    fn meta(&self) -> &dyn DedutyPackageMeta;
    fn files(&self) -> &dyn DedutyFileCollection;
    fn lections(&self) -> &[Box<dyn DedutyLection>];
}
