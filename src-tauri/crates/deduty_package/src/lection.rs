use async_trait::async_trait;

use xresult::XResult;

use crate::DedutyFile;


pub trait DedutyLectionMeta: Sync + Send {
    fn name(&self) -> &String;
    fn order(&self) -> u64;
}

#[async_trait]
pub trait DedutyLection: Sync + Send {
    fn id(&self) -> &String;
    fn meta(&self) -> &dyn DedutyLectionMeta;

    async fn file(&self, id: &str) -> XResult<Option<&dyn DedutyFile>>;
    async fn files(&self) -> XResult<&dyn Iterator<Item = &dyn DedutyFile>>;
}
