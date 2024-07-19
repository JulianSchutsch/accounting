use std::collections::HashMap;

use crate::book::*;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct EntryKey {
    pub date: Date,
    pub ledger_id: LedgerId
}
type BookEntries<'l> = std::collections::BTreeMap<EntryKey, Vec<BookAccountEntry<'l>>>;
pub type BookAccountEntriesIter<'l, 's> = std::collections::btree_map::Iter<'s, EntryKey, Vec<BookAccountEntry<'l>>>;

type ValueEntries<'l> = std::collections::BTreeMap<EntryKey, Vec<BookAccountValueEntry<'l>>>;

pub struct BookAccounts<'l> {
    currency: Currency,
    entries: BookEntries<'l>,
    values: ValueEntries<'l>,
    pub naming: HashMap<BookAccountId, String>
}

impl<'s> BookAccounts<'s> {
    pub fn new<'l>(currency: Currency) -> BookAccounts<'l> {
        BookAccounts{
            currency,
            entries: BookEntries::new(),
            values: ValueEntries::new(),
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
                println!("Entry: {:?} {} {:?} {:?}", entry_key.ledger_id, entry_key.date, entry.account, entry.amount);
            }
        }
    }

    pub fn add_entry<'l: 's>(&mut self, ledger_id: LedgerId, date: Date, source: &'l Event,  account: BookAccountId, amount: BookAccountAmount) {
        let entry = BookAccountEntry{ source, account, amount };
        self.entries.entry(EntryKey{date, ledger_id}).and_modify(|e| e.push(entry)).or_insert(vec![entry]);
    }

    pub fn add_value<'l: 's>(&mut self, ledger_id: LedgerId, date: Date, source: &'l Event, account: BookAccountId, amount: Amount) {
        let entry = BookAccountValueEntry{ source, account, amount };
        self.values.entry(EntryKey{date, ledger_id}).and_modify(|e| e.push(entry)).or_insert(vec![entry]);
    }
}
