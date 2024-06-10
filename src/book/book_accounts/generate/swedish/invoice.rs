use serde_json::to_string;
use crate::book::*;

use super::params::Params;
use super::ids;

fn add_sweden(p: Params<Invoice>) -> BookResult {

    let amounts = p.event.amounts.convert_into_book_currency(p.event.date, &p.import.exchange_rates)?;
    p.accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::CLAIMS_FROM_CUSTOMERS, BookAccountAmount::Credit(amounts.total));

    for (&category, &amount) in amounts.iter() {
        let (incoming_moms_account, moms_factor) = ids::invoice_moms(category, amounts.reverse_charge)?;
        let invoice_account = ids::invoice_account(category)?;
        let moms = moms_factor*amount;
        p.accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, incoming_moms_account, BookAccountAmount::Debit(moms));
        p.accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, invoice_account, BookAccountAmount::Debit(amount));
    }

    for payment in p.event.payments.iter_inverse_date(p.event.date) {
        let account = p.bank_accounts.get_account_by_references(&BankAccountReferences::new_from_single(payment.account.clone()))
            .ok_or_else(|| BookError::new(format!("Cannot find bank account {} for payment for {}", payment.account, p.event.id)))?;
        let amount = payment.amount;
        p.accounts.add_entry(p.ledger_id, payment.date, &p.event_ref, ids::CLAIMS_FROM_CUSTOMERS, BookAccountAmount::Debit(amount));
        match account.account_type {
            BankAccountType::Privat => {
                p.accounts.add_entry(p.ledger_id, payment.date, &p.event_ref, ids::DEBT_TO_PRIVATE, BookAccountAmount::Credit(amount));
            }
            BankAccountType::Account => {
                p.accounts.add_entry(p.ledger_id, payment.date, &p.event_ref, ids::COMPANY_BANK_ACCOUNT, BookAccountAmount::Credit(amount));
            }
        }
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
