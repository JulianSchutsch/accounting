use crate::book::*;

use super::ids;
use super::active_associables::*;
use super::params::*;

pub fn add_company_transaction(ledger_id: LedgerId, event: &Transaction, first: &phases::First, book_accounts: &mut BookAccounts) -> BookResult {
    let book_amount = BookAccountAmount::from_signed_amount(event.amount);
    book_accounts.add_entry(ledger_id, event.date, &event.id, ids::COMPANY_BANK_ACCOUNT, book_amount);
    book_accounts.add_entry(ledger_id, event.date, &event.id, ids::COMPANY_BANK_TRANSACTIONS, book_amount.invert());
    Ok(())
}

pub fn add<'p>(ledger_id: LedgerId, event: &Transaction, p: &mut Params<'p>, associables: &mut ActiveAssociables<'p>) -> BookResult {
    if let BankAccountReference::Skatteverket(_) = event.account {
        if event.references.references.contains("Inbetalning bokförd") {
            return add_company_transaction(ledger_id, event, p.first, &mut p.book);
        }
    }
    if p.first.settings.account_transaction_match(&event.references).is_some() {
        return add_company_transaction(ledger_id, event, p.first, &mut p.book);
    }
    if !associables.transactions.associate(ledger_id, event, p)? {
        println!("Not associated transaction {:?}", event);
    }
    Ok(())
}