use serde::Serialize;

use crate::file::serde::DedutyFileCollection;

#[derive(Serialize)]
pub struct DedutyLectionMeta {
    pub name: String
}

#[derive(Serialize)]
pub struct DedutyLection {
    pub id: String,
    pub name: String,
    pub files: DedutyFileCollection
}
