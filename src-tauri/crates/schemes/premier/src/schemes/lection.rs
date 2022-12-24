use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct PremierLectionPage {
    pub relative: String
}


#[derive(Deserialize, Debug)]
pub struct PremierLectionMeta {
    pub name: String,
    pub order: u64,
    pub pages: Option<Vec<PremierLectionPage>>
}


#[derive(Deserialize, Debug)]
pub struct PremierLection {
    pub lection: PremierLectionMeta
}
