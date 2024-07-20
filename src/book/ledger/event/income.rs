use crate::book::*;

#[derive(Debug, serde::Deserialize)]
pub struct Income {
  pub id: String,
  pub date: Date,
  pub country: Country,
  pub customer_vat: String,
  pub amounts: CategorizedAmounts,
  pub description: String,
  pub payment: Vec<Payment>
}
