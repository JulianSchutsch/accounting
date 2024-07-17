use crate::book::*;

use super::ids;
use super::params::Params;

pub fn add(p: Params<Fine>) -> BookResult {
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::FINES, BookAccountAmount::Debit(p.event.amount));
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::COMPANY_BANK_ACCOUNT, BookAccountAmount::Credit(p.event.amount));
    Ok(())
}