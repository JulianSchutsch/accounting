use crate::book::*;
use crate::book::book::tools::period_sum;

use super::ids;
use super::params::*;

pub fn add(_ledger_id: LedgerId, event: &CheckPeriod, p: &mut Params) -> BookResult {
    let main_currency_sum = period_sum(&p.book, event.period.extend_to_min(), BookIdRange::single(ids::COMPANY_BANK_ACCOUNT))?;
    let main_currency_bank = p.first.bank_accounts.sum_latest_values(p.first.exchange_rates.book_currency, event.period.end);
    if !almost_equal(main_currency_sum, main_currency_bank) {
        return Err( BookError::new(format!("Difference in book currency, month {}", event.period.extend_to_min())));
    }
    Ok(())
}