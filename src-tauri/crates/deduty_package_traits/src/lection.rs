use crate::file::DedutyFileCollection;


pub trait DedutyLectionMeta: Sync + Send {
    fn name(&self) -> String;
    fn order(&self) -> u64;
}

pub trait DedutyLection: Sync + Send {
    fn id(&self) -> String;
    fn meta(&self) -> &dyn DedutyLectionMeta;
    fn files(&self) -> &dyn DedutyFileCollection;
}
