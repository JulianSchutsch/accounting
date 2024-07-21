use std::collections::BTreeMap;
use crate::book::*;

#[derive(Debug, serde::Deserialize)]
pub struct Shares {
    pub id: String,
    pub date: Date,
    pub amount: Amount,
    pub owners: BTreeMap<String, u32>,
    pub payment: Vec<Payment>
}