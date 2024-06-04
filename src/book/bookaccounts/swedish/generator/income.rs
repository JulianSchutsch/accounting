use crate::book::bookaccounts::{AccountAmount, AccountEntry};
use crate::book::ledger::*;
use crate::book::types::*;

use crate::book::bookresult::*;
use crate::book::swedish::bookaccounts;

use super::params::Params;

fn add_worldwide(p: Params<Income>) -> BookResult<()> {
    for MomsClassedAmount(moms_perc, amount, moms) in p.event.amount.iter() {
        let book_amount = p.converter.amount_into_book(p.event.date, p.event.currency, *amount)?;
        if(!p.event.reverse_charge) {
            return Err(BookError::new_from_str("Reverse charge required for worldwide at the moment"));
        }
        match p.event.category {
            IncomeCategory::Services => {
                let pseudo_moms = BookAmount(book_amount.0 * 0.25);
                p.accounts.add_entry(p.ledger_id, &p.event_ref, bookaccounts::CLAIMS_TO_CUSTOMERS, AccountAmount::Debit(book_amount));
                p.accounts.add_entry(p.ledger_id, &p.event_ref, bookaccounts::SALES_OF_SERVICES_WORLDWIDE, AccountAmount::Credit(book_amount));
                p.accounts.add_entry(p.ledger_id, &p.event_ref, bookaccounts::OUTGOING_MOMS_REVERSE_CHARGE_25PERC, AccountAmount::Debit(pseudo_moms));
                p.accounts.add_entry(p.ledger_id, &p.event_ref, bookaccounts::INCOMING_MOMS_PROCUREMENT_ABROAD, AccountAmount::Credit(pseudo_moms));
            }
        }
    }
    Ok(())
}

pub fn add(p: Params<Income>) -> BookResult<()> {
    assert_eq!(p.event.country.is_eu(), false);
    add_worldwide(p)
}
