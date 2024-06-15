use std::collections::BTreeMap;

use crate::book::*;
use super::bank_account_id::BankAccountId;

type AccountsIter<'l> = std::collections::btree_map::Iter<'l, BankAccountId, BankAccount>;
type Accounts = BTreeMap<BankAccountId, BankAccount>;

pub struct BankAccounts {
    accounts: Accounts,
    next_account_id: BankAccountId,
}

impl BankAccounts {

    pub fn iter(&self) -> AccountsIter {
        return self.accounts.iter();
    }

    pub fn new() -> BankAccounts {
        BankAccounts{
            accounts: BTreeMap::new(),
            next_account_id: BankAccountId(0),
        }
    }

    pub fn get_mut_account_by_references(&mut self, references: &BankAccountReferences) -> Option<&mut BankAccount> {
        for (_, account) in self.accounts.iter_mut() {
            if account.references.matching(references) {
                return Some(account);
            }
        }
        None
    }

    pub fn get_account_by_references(&self, references: &BankAccountReferences) -> Option<&BankAccount> {
        for (_, account) in self.accounts.iter() {
            if account.references.matching(references) {
                return Some(account);
            }
        }
        None
    }

    pub fn add_account(&mut self, references: BankAccountReferences, account_type: BankAccountType, currency: Currency, initial_value: Amount) -> BookResult {
        if let Some(account) = self.get_mut_account_by_references(&references) {
            return Err(BookError::new(format!("Account with references {} cannot be created twice", references)));
        }
        let account_id = self.next_account_id.increase();
        self.accounts.insert(account_id, BankAccount::new(account_type, initial_value, currency, references));
        Ok(())
    }

}