use crate::book::types::*;

#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub enum Category {
    #[serde(rename="services")]
    Services,
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
    pub category: Category,
    pub amount: Vec<MomsClassedAmount>,
    pub description: String,
}
