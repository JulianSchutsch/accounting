use crate::book::*;

#[derive(Debug, Clone)]
pub struct BookValueEntry {
    pub source_desc: String,
    pub account: BookId,
    pub amount: Amount
}
