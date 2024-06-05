use crate::book::book_accounts::AccountAmount;
use crate::book::book_result::*;
use crate::book::ledger::*;
use crate::book::types::*;

use super::params::Params;
use super::ids;

fn add_sweden(p: Params<Invoice>) -> BookResult<()> {
    if(p.event.reverse_charge) {
        return Err(BookError::new("Reverse charge not supported within sweden"));
    }
    for MomsClassedAmount(moms_perc, amount, moms) in p.event.amount.iter() {
        let book_amount = p.import.exchange_rates.amount_into_book(p.event.date, p.event.currency, *amount)?;
        let book_moms = p.import.exchange_rates.moms_into_book(p.event.date, p.event.currency, *moms)?;
        p.accounts.add_entry(p.ledger_id, &p.event_ref, ids::CLAIMS_FROM_CUSTOMERS, AccountAmount::Debit(book_amount));
        p.accounts.add_entry(p.ledger_id, &p.event_ref, ids::INCOMING_MOMS, AccountAmount::Debit(book_moms));
    }
    Ok(())
}

fn add_eu(p: Params<Invoice>) -> BookResult<()> {
    panic!("Not implemented")
}

fn add_worldwide(p: Params<Invoice>) -> BookResult<()> {
    panic!("Not implemented")
}

pub fn add(p: Params<Invoice>) -> BookResult<()> {
    if(p.event.country.is_eu()) {
        if(p.event.country==Country::Sweden) {
            add_sweden(p)
        } else {
            add_eu(p)
        }
    } else {
        add_worldwide(p)
    }
}
