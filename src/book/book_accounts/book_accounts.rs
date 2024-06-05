use std::collections::HashMap;
use crate::book::ledger::*;
use crate::book::types::*;

use super::book_account_id::BookAccountId;

#[derive(Debug, Clone, Copy)]
pub enum AccountAmount {
    Debit(BookAmount),
    Credit(BookAmount)
}

impl AccountAmount {
    pub fn plain_amount(&self) -> BookAmount {
        match self {
            AccountAmount::Credit(a) => *a,
            AccountAmount::Debit(a) => *a,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AccountEntry<'l> {
    pub source: &'l Event,
    pub account: BookAccountId,
    pub amount: AccountAmount
}

type EntryKey = (Date, LedgerId);
type Entries<'l> = std::collections::BTreeMap<EntryKey, Vec<AccountEntry<'l>>>;
type EntriesIter<'l, 's> = std::collections::btree_map::Iter<'s, EntryKey, Vec<AccountEntry<'l>>>;

pub struct BookAccounts<'l> {
    currency: Currency,
    entries: Entries<'l>,
    pub naming: HashMap<BookAccountId, String>
}

impl<'s> BookAccounts<'s> {
    pub fn new<'l>(currency: Currency) -> BookAccounts<'l> {
        BookAccounts{
            currency,
            entries: Entries::new(),
            naming: HashMap::new()
        }
    }

    pub fn set_account_name(&mut self, id: BookAccountId, name: &str) {
        self.naming.insert(id, name.to_string());
    }

    pub fn iter(&self) -> EntriesIter { self.entries.iter() }

    pub fn print(&self) {
        for ((date, ledger_id), entry_list) in self.iter() {
            for entry in entry_list.iter() {
                println!("Entry: {:?} {} {:?} {:?} pga={}", ledger_id, date, entry.account, entry.amount, entry.source.id());
            }
        }
    }

    pub fn add_entry<'l: 's>(&mut self, ledger_id: LedgerId, source: &'l Event,  account: BookAccountId, amount: AccountAmount) {
        let entry = AccountEntry{
            source: source,
            account: account,
            amount: amount
        };
        self.entries.entry((source.date(), ledger_id)).and_modify(|e| e.push(entry)).or_insert(vec![entry]);
    }
}
