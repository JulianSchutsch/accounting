use crate::book::*;

use super::ids;
use super::params::*;

pub fn add(ledger_id: LedgerId, event: &Fine, p: &mut Params) -> BookResult {
    p.book.add_entry(ledger_id, event.date, &event.id, ids::FINES, BookAmount::Debit(event.amount));
    p.book.add_entry(ledger_id, event.date, &event.id, ids::COMPANY_BANK_ACCOUNT, BookAmount::Credit(event.amount));
    Ok(())
}