use crate::book::*;

use super::ids;
use super::params::Params;

pub fn add(ledger_id: LedgerId, event: &VerifyMoms, p: &mut Params) -> BookResult {
    let amount = book::tools::period_sum(
        &p.book,
        event.date.this_month()?,
        ids::MOMS_RANGE,
        BookSideFilter::Both
    )?;
    let book_amount = BookAmount::from_signed_amount(amount);
    p.book.add_entry(ledger_id, event.date, &event.id, ids::MOMS_DEBT, book_amount);
    p.book.add_entry(ledger_id, event.date, &event.id, ids::SHORT_TERM_DEBT_TAXES, book_amount.invert());
    Ok(())
}