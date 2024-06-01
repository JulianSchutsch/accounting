use std::collections::HashMap;
use crate::book::types::*;

#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub enum Category {
    #[serde(rename="services")]
    Services,
}

#[derive(Debug, serde::Deserialize)]
pub struct Invoice {
    pub id: String,
    pub date: Date,
    pub country: Country,
    pub currency: Currency,
    pub reverse_charge: bool,
    pub category: Category,
    pub amount: Vec<MomsClassedAmount>,
    pub description: String,
}
