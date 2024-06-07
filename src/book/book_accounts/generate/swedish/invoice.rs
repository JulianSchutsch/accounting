use serde_json::to_string;
use crate::book::*;

use super::params::Params;
use super::ids;

fn add_sweden(p: Params<Invoice>) -> BookResult<()> {
    if p.event.reverse_charge {
        return Err(BookError::new("Reverse charge not supported within sweden"));
    }

    for cl_amount in p.event.amount.iter() {
        let book_total = p.import.exchange_rates.amount_into_book(p.event.date, p.event.currency, cl_amount.total())?;
        let book_moms = p.import.exchange_rates.moms_into_book(p.event.date, p.event.currency, cl_amount.moms)?;
        p.accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::CLAIMS_FROM_CUSTOMERS, BookAccountAmount::Credit(book_total));
        p.accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::INCOMING_MOMS, BookAccountAmount::Debit(book_moms));
        p.accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::invoice_account(p.event.category), BookAccountAmount::Debit(book_total));
    }

    for payment in p.event.payments.iter() {
        let account = p.bank_accounts.get_account_by_reference(&payment.account)
            .ok_or_else(|| BookError::new(format!("Cannot find bank account {} for payment for {}", payment.account, p.event.id)))?;
        let amount = payment.amount.unwrap_or_else(|| p.event.total_amount);
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
