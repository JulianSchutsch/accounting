use crate::book::*;

#[derive(Debug, serde::Deserialize)]
pub enum PaymentKind {
    #[serde(rename="bank-wire")]
    BankWire,
    #[serde(rename="debt-increase")]
    DebtIncrease,
}

#[derive(Debug, serde::Deserialize)]
pub struct Payment {
    pub id: String,
    pub date: Date,
    pub amount: Option<Amount>,
    pub kind: PaymentKind
}

impl Payment {
    pub fn verify(&self) -> BookResult { Ok(()) }
}