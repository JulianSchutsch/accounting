use crate::book::ledger::*;
use crate::book::types::*;

use crate::book::bookresult::*;
use crate::book::swedish::accounts;

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
                p.accounts.add_entry_debet(&p.event_ref, accounts::CLAIMS_TO_CUSTOMERS, book_amount);
                p.accounts.add_entry_kredit(&p.event_ref, accounts::SALES_OF_SERVICES_WORLDWIDE, book_amount);
                p.accounts.add_entry_debet(&p.event_ref, accounts::OUTGOING_MOMS_REVERSE_CHARGE_25PERC, pseudo_moms);
                p.accounts.add_entry_kredit(&p.event_ref, accounts::INCOMING_MOMS_PROCUREMENT_ABROAD, pseudo_moms);
            }
        }
    }
    Ok(())
}

pub fn add(p: Params<Income>) -> BookResult<()> {
    assert_eq!(p.event.country.is_eu(), false);
    add_worldwide(p)
}
