use crate::book::*;

use super::payment_kind::PaymentKind;

#[derive(Debug, serde::Deserialize)]
pub struct InvoicePayment {
    pub id: String,
    pub date: Date,
    pub amount: Option<Amount>,
    pub kind: PaymentKind,
    pub account: BankAccountReference
}

impl InvoicePayment {
    pub fn verify(&self) -> BookResult { Ok(()) }
}