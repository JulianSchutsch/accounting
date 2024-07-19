use crate::book::*;

#[derive(Debug, Clone, Copy)]
pub struct BookAccountValueEntry<'l> {
    pub source: &'l Event,
    pub account: BookAccountId,
    pub amount: Amount
}
