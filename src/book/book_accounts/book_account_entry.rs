use crate::book::*;

#[derive(Debug, Clone, Copy)]
pub struct BookAccountEntry<'l> {
    pub source: &'l Event,
    pub account: BookAccountId,
    pub amount: BookAccountAmount
}
