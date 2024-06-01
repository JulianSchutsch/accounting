use crate::book::bookresult::*;
use crate::book::ledger::*;
use crate::book::types::*;

use crate::book::swedish::accounts;

use super::params::Params;

fn add_sweden(p: Params<Invoice>) -> BookResult<()> {
    if(p.event.reverse_charge) {
        return Err(BookError::new_from_str("Reverse charge not supported within sweden"));
    }
    for MomsClassedAmount(moms_perc, amount, moms) in p.event.amount.iter() {
        let book_amount = p.converter.amount_into_book(p.event.date, p.event.currency, *amount)?;
        let book_moms = p.converter.moms_into_book(p.event.date, p.event.currency, *moms)?;
        p.accounts.add_entry_debet(&p.event_ref, accounts::CLAIMS_FROM_CUSTOMERS, book_amount);
        p.accounts.add_entry_debet(&p.event_ref, accounts::INCOMING_MOMS, book_moms);
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
