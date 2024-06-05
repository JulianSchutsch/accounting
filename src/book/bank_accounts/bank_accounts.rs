use std::collections::{BTreeMap, HashSet};
use crate::book::types::*;

use super::bank_account_id::BankAccountId;
use super::bank_account_reference::BankAccountReference;

pub struct BankTransaction {
    account: BankAccountId,
    amount: Amount,
    currency: Currency,
    references: Vec<String>
}

pub struct BankPeriod {
    account_id: BankAccountId,
    period: Period,
    filename: String
}

pub struct BankAccount {
    references: std::collections::HashSet<BankAccountReference>
}

pub struct BankAccounts {
    accounts: BTreeMap<BankAccountId, BankAccount>,
    next_account_id: BankAccountId,
    periods: Vec<BankPeriod>,
    pub transactions: BTreeMap<Date, Vec<BankTransaction>>,
}

impl BankAccounts {

    pub fn new() -> BankAccounts {
        BankAccounts{
            accounts: BTreeMap::new(),
            next_account_id: BankAccountId(0),
            periods: Vec::new(),
            transactions: BTreeMap::new()
        }
    }

    pub fn add_period(&mut self, account_id: BankAccountId, period: Period, filename: String) {
        self.periods.push(BankPeriod{ account_id, period, filename });
    }

    pub fn find_account(&self, reference: BankAccountReference) -> Option<BankAccountId> {
        for (account_id, account) in self.accounts.iter() {
            if account.references.contains(&reference) {
                return Some(account_id.clone());
            }
        }
        None
    }

    pub fn find_account_by_any_of(&self, references: &HashSet<BankAccountReference>) -> Option<BankAccountId> {
        for (account_id, account) in self.accounts.iter() {
            if !account.references.is_disjoint(references) {
                return Some(account_id.clone());
            }
        }
        None
    }

    pub fn ensure_account(&mut self, references: HashSet<BankAccountReference>) -> BankAccountId {
        if let Some(account_id) = self.find_account_by_any_of(&references) {
            self.accounts.entry(account_id)
                .and_modify(|entry| entry.references.extend(references.into_iter()));
            return account_id
        }
        self.next_account_id.increase()
    }

}