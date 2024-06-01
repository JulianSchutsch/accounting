use crate::book::types::*;

#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize)]
pub enum IncomeCategory {
    #[serde(rename="services")]
    Services,
}

fn default_reverse_charge() -> bool {
  false
}

#[derive(Debug, serde::Deserialize)]
pub struct Income {
  pub id: String,
  pub date: Date,
  pub country: Country,
  #[serde(rename="customer-vat")]
  pub customer_vat: String,
  pub currency: Currency,
  #[serde(default="default_reverse_charge")]
  pub reverse_charge: bool,
  pub category: IncomeCategory,
  pub amount: Vec<MomsClassedAmount>,
  pub description: String,
}