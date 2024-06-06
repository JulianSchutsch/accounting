use crate::book::*;

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
}

impl Invoice {
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