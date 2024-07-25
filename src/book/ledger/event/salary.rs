use crate::book::*;

#[derive(Debug, serde::Deserialize)]
pub struct Salary {
    pub worker: String,
    pub id: String,
    pub date: Date,
    pub declared: Date,
    pub total: Amount,
    pub employee_tax: Amount,
    pub employer_tax: Amount,
    pub payment: Vec<Payment>
}