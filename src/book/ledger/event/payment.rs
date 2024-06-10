use crate::book::*;

#[derive(Debug, serde::Deserialize)]
pub struct Payment {
    pub id: String,
    pub date: Date,
    pub amount: Amount,
    pub account: BankAccountReference
}
