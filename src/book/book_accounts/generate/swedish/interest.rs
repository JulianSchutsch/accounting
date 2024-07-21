use crate::book::*;

use super::ids;
use super::params::*;

fn add_tax_free_interest(ledger_id: LedgerId, event: &Interest, first: &phases::First, book_accounts: &mut BookAccounts) -> BookResult {
    if event.currency!=first.ledger.book_currency {
        return Err(BookError::new("Non taxable interest with different valuta not supported"));
    }
    book_accounts.add_entry(ledger_id, event.date, &event.id, ids::TAX_FREE_INCOME, BookAccountAmount::Credit(event.amount));
    book_accounts.add_entry(ledger_id, event.date, &event.id, ids::COMPANY_BANK_ACCOUNT, BookAccountAmount::Debit(event.amount));
    Ok(())
}

fn add_taxed_interest(ledger_id: LedgerId, event: &Interest, first: &phases::First, book_accounts: &mut BookAccounts) -> BookResult {
    if event.currency==first.ledger.book_currency {
        book_accounts.add_entry(ledger_id, event.date, &event.id, ids::INTEREST_FOR_CURRENT_ASSETS, BookAccountAmount::Credit(event.amount));
        book_accounts.add_entry(ledger_id, event.date, &event.id, ids::COMPANY_BANK_ACCOUNT, BookAccountAmount::Debit(event.amount));
    } else {
        let book_amount = first.exchange_rates.convert_into_book_currency(event.date, event.currency, event.amount)?;
        book_accounts.add_entry(ledger_id, event.date, &event.id, ids::INTEREST_FOR_CURRENT_ASSETS, BookAccountAmount::Credit(book_amount));
        book_accounts.add_entry(ledger_id, event.date, &event.id, ids::COMPANY_CURRENCY_ACCOUNT, BookAccountAmount::Debit(book_amount));
    }
    Ok(())
}

pub fn add(ledger_id: LedgerId, event: &Interest, p: &mut Params) -> BookResult {
    if event.taxable {
        return add_taxed_interest(ledger_id, event, p.first, &mut p.book);
    }
    add_tax_free_interest(ledger_id, event, p.first, &mut p.book)
}