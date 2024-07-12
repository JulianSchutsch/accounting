use crate::book::*;

#[derive(Debug, serde::Deserialize)]
pub enum TaxPaymentKind {
    SocialSecurityTax,
    EmployeeTax
}

#[derive(Debug, serde::Deserialize)]
pub struct TaxPayment {
    pub id: String,
    pub date: Date,
    pub amount: Amount,
    pub kind: TaxPaymentKind
}
