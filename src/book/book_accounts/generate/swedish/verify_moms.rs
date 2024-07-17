use crate::book::*;

use super::ids;
use super::params::Params;

pub fn add(p: Params<VerifyMoms>) -> BookResult {
    let amount = book_accounts::tools::period_sum(
        &p.second.book_accounts,
        p.event.date.previous_month()?,
        ids::MOMS_RANGE,
        BookAccountSide::Both
    )?;
    let book_amount = BookAccountAmount::from_signed_amount(amount);
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::MOMS_DEBT, book_amount);
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::SHORT_TERM_DEBT_TAXES, book_amount.invert());
    Ok(())
}