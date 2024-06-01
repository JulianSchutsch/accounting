use crate::book::ledger::*;
use crate::book::types::*;

use super::accountid::AccountId;

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
    pub account: AccountId,
    pub amount: AccountAmount
}

type EntryKey = (Date, LedgerId);
type Entries<'l> = std::collections::BTreeMap<EntryKey, Vec<AccountEntry<'l>>>;
type EntriesIter<'l, 's> = std::collections::btree_map::Iter<'s, EntryKey, Vec<AccountEntry<'l>>>;

pub struct Accounts<'l> {
    currency: Currency,
    entries: Entries<'l>,
}

impl<'s> Accounts<'s> {
    pub fn new<'l>(currency: Currency) -> Accounts<'l> {
        Accounts{
            currency: currency,
            entries: Entries::new()
        }
    }

    pub fn iter(&self) -> EntriesIter { self.entries.iter() }

    pub fn print(&self) {
        for ((date, ledger_id), entry_list) in self.iter() {
            for entry in entry_list.iter() {
                println!("Entry: {:?} {} {:?} {:?} pga={}", ledger_id, date, entry.account, entry.amount, entry.source.id());
            }
        }
    }

    pub fn add_entry<'l: 's>(&mut self, ledger_id: LedgerId, source: &'l Event,  account: AccountId, amount: AccountAmount) {
        let entry = AccountEntry{
            source: source,
            account: account,
            amount: amount
        };
        self.entries.entry((source.date(), ledger_id)).and_modify(|e| e.push(entry)).or_insert(vec![entry]);
    }
}
