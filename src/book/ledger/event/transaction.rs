use crate::book::*;

#[derive(Debug, serde::Deserialize)]
pub struct Transaction {
    pub id: String,
    pub account: BankAccountReference,
    pub date: Date,
    pub amount: Amount,
    pub currency: Currency,
    pub references: BankTransactionReferences
}
