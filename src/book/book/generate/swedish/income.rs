use std::ffi::c_double;
use crate::book::*;
use crate::book::book::generate::swedish::active_associables::ActiveAssociables;

use super::ids;
use super::params::*;
use super::payment::*;

fn add_moms_worldwide(ledger_id: LedgerId, event: &Income, first: &phases::First, book: &mut Book) -> BookResult {
    let amounts = event.amounts.convert_into_book_currency(event.date, &first.exchange_rates)?;

    let mut accumulated_moms = Amount(0.0);
    for (&category, &amount) in amounts.iter() {
        let (outgoing_moms_account, moms_factor) = ids::income_moms(category, amounts.reverse_charge)?;
        let income_account = ids::income_worldwide_account(category)?;
        let moms = moms_factor * amount;
        accumulated_moms+=moms;
        book.add_entry(ledger_id, event.date, &event.id, income_account, BookAmount::Credit(amount));
        book.add_entry(ledger_id, event.date, &event.id, outgoing_moms_account, BookAmount::Debit(moms));
    }
    if amounts.reverse_charge {
        book.add_entry(ledger_id, event.date, &event.id, ids::INCOMING_MOMS_PROCUREMENT_ABROAD, BookAmount::Credit(accumulated_moms));
    }
    Ok(())
}

pub fn add<'p>(ledger_id: LedgerId, event: &Income, p: &mut Params<'p>, associables: &mut ActiveAssociables<'p>) -> BookResult {
    assert_eq!(event.country.is_eu(), false);
    add_moms_worldwide(ledger_id, event, p.first, &mut p.book).map_err(|e| e.extend("Failed to add world wide income"))?;
    let event_data = PaymentEventData{
        ledger_id,
        date: event.date,
        id: event.id.clone(),
        amount: event.amounts.total,
        currency: event.amounts.currency,
        payments: event.payment.clone(),
        exchange_rate: event.amounts.exchange_rate
    };
    process_payment(event_data, ids::CLAIMS_TO_CUSTOMERS, p, associables)
}
