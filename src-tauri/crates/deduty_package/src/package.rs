use std::collections::HashSet;

use crate::DedutyLection;


pub trait DedutyPackageMeta: Sync + Send {
    fn name(&self) -> &String;
    fn version(&self) -> &String;
    fn language(&self) -> &String;
    fn tags(&self) -> HashSet<&String>;
}

pub trait DedutyPackage: Sync + Send {
    fn id(&self) -> &String;
    fn meta(&self) -> &dyn DedutyPackageMeta;

    fn lection(&self, id: &str) -> Option<&dyn DedutyLection>;
    fn lections(&self) -> &dyn Iterator<Item = &dyn DedutyLection>;
}
