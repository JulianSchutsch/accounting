use crate::book::*;

use super::params::Params;
use super::ids;

pub fn add(p: Params<Shares>) -> BookResult {
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::COMPANY_BANK_ACCOUNT, BookAccountAmount::Debit(p.event.amount));
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::SHARES_CAPITAL, BookAccountAmount::Credit(p.event.amount));
    Ok(())
}