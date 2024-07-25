use crate::book::*;

use super::ids;
use super::params::*;

fn add_tax_free_interest(ledger_id: LedgerId, event: &Interest, first: &phases::First, book: &mut Book) -> BookResult {
    if event.currency!=first.ledger.book_currency {
        return Err(BookError::new("Non taxable interest with different valuta not supported"));
    }
    book.add_entry(ledger_id, event.date, &event.id, ids::TAX_FREE_INCOME, BookAmount::Credit(event.amount));
    book.add_entry(ledger_id, event.date, &event.id, ids::COMPANY_BANK_ACCOUNT, BookAmount::Debit(event.amount));
    Ok(())
}

fn add_taxed_interest(ledger_id: LedgerId, event: &Interest, first: &phases::First, book: &mut Book) -> BookResult {
    if event.currency==first.ledger.book_currency {
        book.add_entry(ledger_id, event.date, &event.id, ids::INTEREST_FOR_CURRENT_ASSETS, BookAmount::Credit(event.amount));
        book.add_entry(ledger_id, event.date, &event.id, ids::COMPANY_BANK_ACCOUNT, BookAmount::Debit(event.amount));
    } else {
        let book_amount = first.exchange_rates.convert_into_book_currency(event.date, event.currency, event.amount, None)?;
        book.add_entry(ledger_id, event.date, &event.id, ids::INTEREST_FOR_CURRENT_ASSETS, BookAmount::Credit(book_amount));
        book.add_entry(ledger_id, event.date, &event.id, ids::COMPANY_CURRENCY_ACCOUNT, BookAmount::Debit(book_amount));
    }
    Ok(())
}

pub fn add(ledger_id: LedgerId, event: &Interest, p: &mut Params) -> BookResult {
    if event.taxable {
        return add_taxed_interest(ledger_id, event, p.first, &mut p.book);
    }
    add_tax_free_interest(ledger_id, event, p.first, &mut p.book)
}