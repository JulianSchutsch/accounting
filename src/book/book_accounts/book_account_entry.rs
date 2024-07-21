use crate::book::*;

#[derive(Debug, Clone)]
pub struct BookAccountEntry {
    pub source_desc: String,
    pub account: BookAccountId,
    pub amount: BookAccountAmount
}
