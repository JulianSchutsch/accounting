use crate::book::*;

#[derive(Debug, serde::Deserialize)]
pub struct BankCost {
    pub id: String,
    pub date: Date,
    pub amount: Amount,
    pub currency: Currency
}
