use crate::book::*;

use super::invoice_payment::InvoicePayment;

#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub enum InvoiceCategory {
    #[serde(rename="software-license")]
    SoftwareLicense,
    #[serde(rename="media-advertisment")]
    MediaAdvertisment,
}

fn default_reverse_charge() -> bool {
    false
}

#[derive(Debug, serde::Deserialize)]
pub struct Invoice {
    pub id: String,
    pub date: Date,
    pub country: Country,
    pub currency: Currency,
    #[serde(default="default_reverse_charge")]
    pub reverse_charge: bool,
    pub category: InvoiceCategory,
    pub amount: Vec<MomsClassedAmount>,
    pub description: String,
    #[serde(skip_deserializing)]
    pub total_amount: Amount,
    #[serde(skip_deserializing)]
    pub payments: Vec<InvoicePayment>
}

impl Invoice {
    pub fn verify_and_complete(&mut self) -> BookResult {
        if self.reverse_charge {
            if self.amount.iter().any(|v | v.moms.0!=0.0) {
                return Err(BookError::new("Moms cannot be non zero for reverse charge"));
            }
        }
        if self.amount.iter().any(|v| !v.verify()) {
            return Err(BookError::new("Moms calculation off"));
        }
        self.total_amount = self.amount.iter().fold(Amount(0.0), |acc, MomsClassedAmount{moms_percent:_, amount, moms}| {
            *amount + *moms
        });
        Ok(())
    }
}