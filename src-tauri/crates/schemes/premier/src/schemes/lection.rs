use serde::Deserialize;


#[derive(Deserialize)]
pub struct PremierLectionPage {
    pub relative: String
}


#[derive(Deserialize)]
pub struct PremierLectionMeta {
    pub name: String,
    pub order: u64
}


#[derive(Deserialize)]
pub struct PremierLection {
    pub lection: PremierLectionMeta,
    pub pages: Option<Vec<PremierLectionPage>>
}
