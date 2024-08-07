use std::collections::HashMap;

use crate::book::*;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct EntryKey {
    pub date: Date,
    pub ledger_id: LedgerId
}
type BookEntries = std::collections::BTreeMap<EntryKey, Vec<BookEntry>>;
pub type BookAccountEntriesIter<'s> = std::collections::btree_map::Iter<'s, EntryKey, Vec<BookEntry>>;

type ValueEntries = std::collections::BTreeMap<EntryKey, Vec<BookValueEntry>>;

pub struct Book {
    currency: Currency,
    entries: BookEntries,
    values: ValueEntries,
    pub naming: HashMap<BookId, String>
}

impl Book {
    pub fn new(currency: Currency) -> Book {
        Book{
            currency,
            entries: BookEntries::new(),
            values: ValueEntries::new(),
            naming: HashMap::new()
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.values.clear();
    }

    pub fn set_account_name(&mut self, id: BookId, name: &str) {
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

    pub fn add_entry(&mut self, ledger_id: LedgerId, date: Date, source_desc: &String,  account: BookId, amount: BookAmount) {
        self.entries.entry(EntryKey{date, ledger_id})
            .and_modify(|e| e.push(BookEntry{ source_desc: source_desc.clone(), account, amount }))
            .or_insert(vec![BookEntry{ source_desc: source_desc.clone(), account, amount }]);
    }

    pub fn add_value(&mut self, ledger_id: LedgerId, date: Date, source_desc: &String, account: BookId, amount: Amount) {
        self.values.entry(EntryKey{date, ledger_id})
            .and_modify(|e| e.push(BookValueEntry{ source_desc: source_desc.clone(), account, amount }))
            .or_insert(vec![BookValueEntry{ source_desc: source_desc.clone(), account, amount }]);
    }
}
