use crate::book::*;

#[derive(Debug, serde::Deserialize)]
pub struct Fine {
    pub id: String,
    pub date: Date,
    pub amount: Amount,
}
