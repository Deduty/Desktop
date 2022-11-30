use serde::Serialize;

#[derive(Serialize)]
pub struct DedutyFile {
    pub alias: Option<String>,
    pub location: String,
    pub extension: String
}

#[derive(Serialize)]
pub struct DedutyFileCollection {
    pub files: Vec<DedutyFile>
}
