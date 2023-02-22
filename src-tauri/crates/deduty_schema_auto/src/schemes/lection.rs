use serde::{ Deserialize, Serialize };


#[derive(Debug, Deserialize, Serialize)]
pub struct LectionFiles {
    pub id: Option<String>,
    pub name: Option<String>,
    pub hidden: Option<bool>,
    pub first: Option<Vec<String>>,
    pub last: Option<Vec<String>>
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Lection {
    pub lection: Option<LectionFiles>
}
