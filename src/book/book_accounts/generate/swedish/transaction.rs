use std::ptr::addr_of_mut;
use crate::book::*;

use super::ids;
use super::params::Params;

pub fn add_company_transaction(p: &mut Params<Transaction>) -> BookResult {
    println!("Transaction between accounts {:?} {}", p.event.references, p.event.amount);
    let book_amount = BookAccountAmount::from_signed_amount(p.event.amount);
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::COMPANY_BANK_ACCOUNT, book_amount);
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::COMPANY_BANK_TRANSACTIONS, book_amount.invert());
    Ok(())
}

pub fn add(p: &mut Params<Transaction>) -> BookResult {
    if let BankAccountReference::Skatteverket(_) = p.event.account {
        if p.event.references.references.contains("Inbetalning bokförd") {
            return add_company_transaction(p);
        }
    }
    if p.first.settings.account_transaction_match(&p.event.references).is_some() {
        return add_company_transaction(p);
    }
    if !p.associables.transactions.associate(p.event)? {
        println!("Not associated transaction : {:?}", p.event);
    }
    Ok(())
}