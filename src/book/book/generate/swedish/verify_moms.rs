use crate::book::*;

use super::ids;
use super::params::Params;

pub fn add(ledger_id: LedgerId, event: &VerifyMoms, p: &mut Params) -> BookResult {
    let amount = crate::book::book::tools::extract_sums(ledger_id, event.date, &event.id, &mut p.book, event.date.this_month()?, ids::MOMS_RANGE)?;
    if !amount.almost_zero() {
        let book_amount = BookAmount::from_signed_amount(amount);
        p.book.add_entry(ledger_id, event.date, &event.id, ids::MOMS_DEBT, book_amount);
    }
    Ok(())
}