use crate::book::book_accounts::AccountAmount;
use crate::book::ledger::*;
use crate::book::types::*;

use crate::book::book_result::*;

use super::params::Params;
use super::ids;

fn add_worldwide(p: Params<Income>) -> BookResult<()> {
    for MomsClassedAmount(moms_perc, amount, moms) in p.event.amount.iter() {
        let book_amount = p.import.exchange_rates.amount_into_book(p.event.date, p.event.currency, *amount)?;
        if(!p.event.reverse_charge) {
            return Err(BookError::new("Reverse charge required for worldwide at the moment"));
        }
        match p.event.category {
            IncomeCategory::Services => {
                let pseudo_moms = BookAmount(book_amount.0 * 0.25);
                p.accounts.add_entry(p.ledger_id, &p.event_ref, ids::CLAIMS_TO_CUSTOMERS, AccountAmount::Debit(book_amount));
                p.accounts.add_entry(p.ledger_id, &p.event_ref, ids::SALES_OF_SERVICES_WORLDWIDE, AccountAmount::Credit(book_amount));
                p.accounts.add_entry(p.ledger_id, &p.event_ref, ids::OUTGOING_MOMS_REVERSE_CHARGE_25PERC, AccountAmount::Debit(pseudo_moms));
                p.accounts.add_entry(p.ledger_id, &p.event_ref, ids::INCOMING_MOMS_PROCUREMENT_ABROAD, AccountAmount::Credit(pseudo_moms));
            }
        }
    }
    Ok(())
}

pub fn add(p: Params<Income>) -> BookResult<()> {
    assert_eq!(p.event.country.is_eu(), false);
    add_worldwide(p)
}
