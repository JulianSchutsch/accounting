use crate::book::*;

fn default_reverse_charge() -> bool {
    false
}

#[derive(Debug, serde::Deserialize)]
pub struct Invoice {
    pub id: String,
    pub date: Date,
    pub country: Country,
    pub amounts: CategorizedAmounts,
    pub description: String,
}
