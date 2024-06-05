use crate::book::book_accounts::book_account_id::BookAccountId;
use crate::book::ledger::Event;

use super::book_account_amount::BookAccountAmount;

#[derive(Debug, Clone, Copy)]
pub struct BookAccountEntry<'l> {
    pub source: &'l Event,
    pub account: BookAccountId,
    pub amount: BookAccountAmount
}
