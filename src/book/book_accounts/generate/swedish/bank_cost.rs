use crate::book::*;

use super::ids;
use super::params::*;

pub fn add(ledger_id: LedgerId, event: &BankCost, p: &mut Params) -> BookResult {
    if event.currency==p.first.ledger.book_currency {
        p.book.add_entry(ledger_id, event.date, &event.id, ids::COMPANY_BANK_ACCOUNT, BookAccountAmount::Credit(event.amount));
        p.book.add_entry(ledger_id, event.date, &event.id, ids::BANK_COSTS, BookAccountAmount::Debit(event.amount));
    } else {
        let book_amount = p.first.exchange_rates.convert_into_book_currency(event.date, event.currency, event.amount)?;
        p.book.add_entry(ledger_id, event.date, &event.id, ids::COMPANY_CURRENCY_ACCOUNT, BookAccountAmount::Credit(book_amount));
        p.book.add_entry(ledger_id, event.date, &event.id, ids::BANK_COSTS, BookAccountAmount::Debit(book_amount));
    }
    Ok(())
}