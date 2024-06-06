use std::collections::{BTreeMap, HashSet};
use crate::book::book_result::*;
use crate::book::types::*;

use super::bank_account_id::BankAccountId;
use super::bank_account_references::BankAccountReferences;

#[derive(Clone)]
pub struct BankTransaction {
    amount: Amount,
    references: Vec<String>,
    consumed: bool
}

pub struct BankPeriod {
    account_id: BankAccountId,
    period: Period,
    filename: String
}

#[derive(Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub enum BankAccountType {
    #[serde(rename="account")]
    Account,
    #[serde(rename="credit")]
    Credit
}

pub struct BankAccount {
    account_type: BankAccountType,
    initial_value: Amount,
    currency: Currency,
    references: BankAccountReferences,
    transactions: BTreeMap<Date, Vec<BankTransaction>>,
}

pub struct BankAccounts {
    accounts: BTreeMap<BankAccountId, BankAccount>,
    next_account_id: BankAccountId,
    periods: Vec<BankPeriod>,
}

impl BankAccounts {

    pub fn new() -> BankAccounts {
        BankAccounts{
            accounts: BTreeMap::new(),
            next_account_id: BankAccountId(0),
            periods: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, account_references: BankAccountReferences, date: Date, amount: Amount, references: Vec<String>) -> BookResult {
        if let Some((_, account)) = self.get_mut_account_by_references(&account_references) {
            let transaction = BankTransaction{amount, references, consumed: false};
            account.transactions.entry(date).or_insert({ vec![] }).push(transaction);
            return Ok(());
        }
        Err(BookError::new("Account not found by references"))
    }

    pub fn add_period(&mut self, account_id: BankAccountId, period: Period, filename: String) {
        self.periods.push(BankPeriod{ account_id, period, filename });
    }

    pub fn get_mut_account_by_references(&mut self, references: &BankAccountReferences) -> Option<(BankAccountId, &mut BankAccount)> {
        for (account_id, account) in self.accounts.iter_mut() {
            if account.references.matching(references) {
                return Some((account_id.clone(), account));
            }
        }
        None
    }

    pub fn ensure_account(&mut self, references: BankAccountReferences, account_type: BankAccountType, o_currency: Option<Currency>, o_initial_value: Option<Amount>) -> BookResult<BankAccountId> {
        if let Some((account_id, account)) = self.get_mut_account_by_references(&references) {
            if account.account_type!=account_type {
                return Err(BookError::new("Unexpected change of account type"));
            }
            if o_currency.is_some() {
                return Err(BookError::new("Account cannot change its currency"));
            }
            if o_initial_value.is_some() {
                return Err(BookError::new("Cannot change accounts initial value"));
            }
            account.references.extend(references);
            return Ok(account_id);
        }
        if let (Some(initial_value), Some(currency))= (o_initial_value, o_currency) {
            let account_id = self.next_account_id.increase();
            self.accounts.insert(account_id, BankAccount{ account_type, initial_value, currency, references, transactions: BTreeMap::new() });
            Ok(account_id)
        } else {
            Err(BookError::new("New account needs initial value"))
        }
    }

}