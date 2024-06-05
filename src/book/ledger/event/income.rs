use crate::book::{BookError, BookResult};
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

impl Income {
  pub fn verify(&self) -> BookResult {
    if self.reverse_charge {
      if self.amount.iter().any(|v | v.moms.0!=0.0) {
        return Err(BookError::new("Moms cannot be non zero for reverse charge"));
      }
    }
    if self.amount.iter().any(|v| !v.verify()) {
      return Err(BookError::new("Moms calculation off"));
    }
    Ok(())
  }
}