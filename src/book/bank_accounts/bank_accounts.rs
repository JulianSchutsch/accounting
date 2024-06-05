use std::collections::{BTreeMap, HashSet};
use crate::book::book_result::*;
use crate::book::types::*;

use super::bank_account_id::BankAccountId;
use super::bank_account_reference::BankAccountReference;

pub struct BankTransaction {
    account: BankAccountId,
    amount: Amount,
    references: Vec<String>
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
    references: HashSet<BankAccountReference>
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

    pub fn ensure_account(&mut self, references: HashSet<BankAccountReference>, account_type: BankAccountType, o_currency: Option<Currency>, o_initial_value: Option<Amount>) -> BookResult<BankAccountId> {
        if let Some(account_id) = self.find_account_by_any_of(&references) {
            let account = self.accounts.get_mut(&account_id).unwrap();
            if account.account_type!=account_type {
                return Err(BookError::new("Unexpected change of account type"));
            }
            if o_currency.is_some() {
                return Err(BookError::new("Account cannot change its currency"));
            }
            if o_initial_value.is_some() {
                return Err(BookError::new("Cannot change accounts initial value"));
            }
            account.references.extend(references.into_iter());
            return Ok(account_id);
        }
        if let (Some(initial_value), Some(currency))= (o_initial_value, o_currency) {
            let account_id = self.next_account_id.increase();
            self.accounts.insert(account_id, BankAccount{ account_type, initial_value, currency, references });
            Ok(account_id)
        } else {
            Err(BookError::new("New account needs initial value"))
        }
    }

}