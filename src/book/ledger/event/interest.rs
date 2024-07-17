use crate::book::*;

#[derive(Debug, serde::Deserialize)]
pub struct Interest {
    pub id: String,
    pub date: Date,
    pub amount: Amount,
    pub taxable: bool
}
