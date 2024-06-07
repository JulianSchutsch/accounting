use std::collections::HashMap;

use crate::book::*;

use super::book_account_id::BookAccountId;
use super::book_account_entry::BookAccountEntry;
use super::book_account_amount::BookAccountAmount;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct EntryKey {
    pub date: Date,
    pub ledger_id: LedgerId
}
type Entries<'l> = std::collections::BTreeMap<EntryKey, Vec<BookAccountEntry<'l>>>;
pub type BookAccountEntriesIter<'l, 's> = std::collections::btree_map::Iter<'s, EntryKey, Vec<BookAccountEntry<'l>>>;

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

    pub fn iter(&self) -> BookAccountEntriesIter { self.entries.iter() }

    pub fn print(&self) {
        for (entry_key, entry_list) in self.iter() {
            for entry in entry_list.iter() {
                println!("Entry: {:?} {} {:?} {:?} pga={}", entry_key.ledger_id, entry_key.date, entry.account, entry.amount, entry.source.id());
            }
        }
    }

    pub fn add_entry<'l: 's>(&mut self, ledger_id: LedgerId, date: Date, source: &'l Event,  account: BookAccountId, amount: BookAccountAmount) {
        let entry = BookAccountEntry{ source, account, amount };
        self.entries.entry(EntryKey{date, ledger_id}).and_modify(|e| e.push(entry)).or_insert(vec![entry]);
    }
}
