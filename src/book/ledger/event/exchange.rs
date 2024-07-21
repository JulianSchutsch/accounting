use crate::book::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Exchange {
    pub id: String,
    pub date: Date,
    pub currency: Currency,
    pub amount: Amount
}
