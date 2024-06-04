use crate::book::Currency;

#[derive(serde::Deserialize)]
pub struct Riksbank {
    pub currency: Currency,
    pub file: String,
}
