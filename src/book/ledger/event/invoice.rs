use crate::book::*;

#[derive(Debug, serde::Deserialize)]
pub struct Invoice {
    pub id: String,
    pub date: Date,
    pub country: Country,
    pub amounts: CategorizedAmounts,
    pub description: String,
    pub payment: Vec<Payment>
}
