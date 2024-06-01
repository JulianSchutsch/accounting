use crate::book::types::*;

#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize)]
pub enum IncomeCategory {
    #[serde(rename="services")]
    Services,
}

#[derive(Debug, serde::Deserialize)]
pub struct Income {
  pub id: String,
  pub date: Date,
  pub country: Country,
  #[serde(rename="customer-vat")]
  pub customer_vat: String,
  pub currency: Currency,
  pub reverse_charge: bool,
  pub category: IncomeCategory,
  pub amount: Vec<MomsClassedAmount>,
  pub description: String,
}