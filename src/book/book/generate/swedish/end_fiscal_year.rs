use crate::book::*;
use crate::book::book::tools::period_sum;

use super::ids;
use super::params::Params;

pub fn add(ledger_id: LedgerId, event: &EndFiscalYear, p: &mut Params) -> BookResult {
    let book_sum = period_sum(&p.book, Period::from_dates(Date::MIN, event.fiscal_year.end), BookIdRange::new(ids::COMPANY_CURRENCY_ACCOUNT, ids::COMPANY_CURRENCY_ACCOUNT))?;
    let mut exchanged_sums = Amount(0.0);
    for &currency in ALL_CURRENCIES.iter() {
        if currency!=p.first.settings.book_currency {
            let bank_sum = p.first.bank_accounts.sum_latest_values(currency, event.fiscal_year.end);
            let exchanged_sum = p.first.exchange_rates.convert_into_book_currency(event.fiscal_year.end, currency, bank_sum, None)?;
            exchanged_sums += exchanged_sum;
        }
    }
    let loss = book_sum-exchanged_sums;
    let book_loss = BookAmount::from_signed_amount(loss);
    p.book.add_entry(ledger_id, event.fiscal_year.end, &event.id, ids::EXCHANGE_RATE_DIFFERENCES, book_loss);
    p.book.add_entry(ledger_id, event.fiscal_year.end, &event.id, ids::COMPANY_CURRENCY_ACCOUNT, book_loss.invert());
    Ok(())
}