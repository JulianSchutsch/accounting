use crate::book::*;

use super::ids;
use super::params::*;

pub fn add(ledger_id: LedgerId, event: &BankCost, p: &mut Params) -> BookResult {
    if event.currency==p.first.ledger.book_currency {
        p.book.add_entry(ledger_id, event.date, &event.id, ids::COMPANY_BANK_ACCOUNT, BookAmount::Credit(event.amount));
        p.book.add_entry(ledger_id, event.date, &event.id, ids::BANK_COSTS, BookAmount::Debit(event.amount));
    } else {
        let book_amount = p.first.exchange_rates.convert_into_book_currency(event.date, event.currency, event.amount, None)?;
        p.book.add_entry(ledger_id, event.date, &event.id, ids::COMPANY_CURRENCY_ACCOUNT, BookAmount::Credit(book_amount));
        p.book.add_entry(ledger_id, event.date, &event.id, ids::BANK_COSTS, BookAmount::Debit(book_amount));
    }
    Ok(())
}