use crate::book::*;

use super::ids;
use super::params::Params;

pub fn add(p: Params<Interest>) -> BookResult {
    if p.event.taxable {
        return Err(BookError::new("Taxable interest not implemented yet"));
    }
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::TAX_FREE_INCOME, BookAccountAmount::Credit(p.event.amount));
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::COMPANY_BANK_ACCOUNT, BookAccountAmount::Debit(p.event.amount));
    Ok(())
}