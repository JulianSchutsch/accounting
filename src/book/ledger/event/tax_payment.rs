use crate::book::*;

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub enum TaxPaymentKind {
    SocialSecurityTax,
    EmployeeTax,
    CompanyTax,
    Moms
}

#[derive(Debug, serde::Deserialize)]
pub struct TaxPayment {
    pub id: String,
    pub date: Date,
    pub amount: Amount,
    pub kind: TaxPaymentKind
}
