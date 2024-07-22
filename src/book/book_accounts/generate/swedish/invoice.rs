use crate::book::*;
use crate::book::book_accounts::generate::swedish::active_associables::ActiveAssociables;

use super::ids;
use super::params::*;
use super::payment::*;

fn add_moms_sweden(ledger_id: LedgerId, event: &Invoice, first: &phases::First, book_accounts: &mut BookAccounts) -> BookResult {
    let amounts = event.amounts.convert_into_book_currency(event.date, &first.exchange_rates)?;
    // TODO: Reverse charge in sweden?

    for (&category, &amount) in amounts.iter() {
        let (incoming_moms_account, moms_factor) = ids::invoice_moms(category)?;
        let invoice_account = ids::invoice_account(category)?;
        let moms = moms_factor*amount;
        book_accounts.add_entry(ledger_id, event.date, &event.id, incoming_moms_account, BookAccountAmount::Debit(moms));
        book_accounts.add_entry(ledger_id, event.date, &event.id, invoice_account, BookAccountAmount::Debit(amount));
    }
    Ok(())
}

fn add_moms_europa(ledger_id: LedgerId, event: &Invoice, first: &phases::First, book_accounts: &mut BookAccounts) -> BookResult {
    let amounts = event.amounts.convert_into_book_currency(event.date, &first.exchange_rates)?;

    for (&category, &amount) in amounts.iter() {
        let invoice_account = ids::invoice_account(category)?;
        book_accounts.add_entry(ledger_id, event.date, &event.id, invoice_account, BookAccountAmount::Debit(amount));
        if amounts.reverse_charge {
            book_accounts.add_entry(ledger_id, event.date, &event.id, ids::invoice_moms_reverse_charge(category)?, BookAccountAmount::Credit(amount));
        }
    }
    Ok(())
}

fn add_moms(ledger_id: LedgerId, event: &Invoice, first: &phases::First, book_accounts: &mut BookAccounts) -> BookResult {
    if event.country.is_eu() {
        if event.country==Country::Sweden {
            add_moms_sweden(ledger_id, event, first, book_accounts)
        } else {
            add_moms_europa(ledger_id, event, first, book_accounts)
        }
    } else {
        panic!("Not implemented");
    }
}

pub fn add(ledger_id: LedgerId, event: &Invoice, p: &mut Params, associables: &mut ActiveAssociables) -> BookResult {
    add_moms(ledger_id, event, p.first, &mut p.book)?;
    let event_data = PaymentEventData {
        ledger_id,
        date: event.date,
        id: event.id.clone(),
        amount: -event.amounts.total,
        currency: event.amounts.currency,
        payments: event.payment.clone()
    };
    process_payment(event_data, ids::CLAIMS_FROM_SUPPLIERS, p, associables)?;
    Ok(())
}