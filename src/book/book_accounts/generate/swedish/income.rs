use crate::book::*;

use super::params::Params;
use super::ids;

fn add_worldwide(p: Params<Income>) -> BookResult {
    let amounts = p.event.amounts.convert_into_book_currency(p.event.date, &p.import.exchange_rates)?;

    p.accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::CLAIMS_TO_CUSTOMERS, BookAccountAmount::Debit(amounts.total));

    let mut accumulated_moms = Amount(0.0);
    for (&category, &amount) in amounts.iter() {
        let (outgoing_moms_account, moms_factor) = ids::income_moms(category, amounts.reverse_charge)?;
        let income_account = ids::income_worldwide_account(category)?;
        let moms = moms_factor * amount;
        accumulated_moms+=moms;
        p.accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, income_account, BookAccountAmount::Credit(amount));
        p.accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, outgoing_moms_account, BookAccountAmount::Debit(moms));
    }
    if amounts.reverse_charge {
        p.accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::INCOMING_MOMS_PROCUREMENT_ABROAD, BookAccountAmount::Credit(accumulated_moms));
    }
    Ok(())
}

pub fn add(p: Params<Income>) -> BookResult {
    assert_eq!(p.event.country.is_eu(), false);
    add_worldwide(p).map_err(|e| e.extend("Failed to add world wide income"))
}
