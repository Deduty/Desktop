use serde::{ Deserialize, Serialize };


#[derive(Debug, Deserialize, Serialize)]
pub struct PremierLectionPage {
    pub relative: String
}


#[derive(Debug, Deserialize, Serialize)]
pub struct PremierLectionMeta {
    pub id: Option<String>,
    pub name: String,
    pub order: u64,
    pub pages: Option<Vec<PremierLectionPage>>
}


#[derive(Debug, Deserialize, Serialize)]
pub struct PremierLection {
    pub lection: PremierLectionMeta
}
