use crate::book::*;

use super::params::Params;
use super::ids;

fn add_sweden(p: Params<Invoice>) -> BookResult {

    let amounts = p.event.amounts.convert_into_book_currency(p.event.date, &p.first.exchange_rates)?;
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::CLAIMS_FROM_CUSTOMERS, BookAccountAmount::Credit(amounts.total));

    for (&category, &amount) in amounts.iter() {
        let (incoming_moms_account, moms_factor) = ids::invoice_moms(category, amounts.reverse_charge)?;
        let invoice_account = ids::invoice_account(category)?;
        let moms = moms_factor*amount;
        p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, incoming_moms_account, BookAccountAmount::Debit(moms));
        p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, invoice_account, BookAccountAmount::Debit(amount));
    }
    Ok(())
}

fn add_eu(p: Params<Invoice>) -> BookResult<()> {
    panic!("Not implemented")
}

fn add_worldwide(_p: Params<Invoice>) -> BookResult<()> {
    panic!("Not implemented")
}

pub fn add(p: Params<Invoice>) -> BookResult<()> {
    if p.event.country.is_eu() {
        if p.event.country==Country::Sweden {
            add_sweden(p)
        } else {
            add_eu(p)
        }
    } else {
        add_worldwide(p)
    }
}
