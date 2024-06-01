use crate::book::ledger::Event;
use crate::book::types::*;

use super::id::Id;

pub struct Entry<'l> {
    source: &'l Event,
    account: Id,
    debet: Option<BookAmount>,
    kredit: Option<BookAmount>,
}

type Entries<'l> = std::collections::BTreeMap<Date, Entry<'l>>;
type EntriesIter<'l, 's> = std::collections::btree_map::Iter<'s, Date, Entry<'l>>;

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
        for (date, entry) in self.iter() {
            println!("Entry: {} {:?} {:?} {:?} pga={}", date, entry.account, entry.debet, entry.kredit, entry.source.id());
        }
    }

    pub fn add_entry_debet<'l: 's>(&mut self, source: &'l Event, account: Id, debet: BookAmount) {
        let date = source.date().clone();
        self.entries.insert(date, Entry{
            source: source,
            account: account,
            debet: Some(debet),
            kredit: None,
        });
    }
    pub fn add_entry_kredit<'l: 's>(&mut self, source: &'l Event, account: Id, kredit: BookAmount) {
        let date = source.date().clone();
        self.entries.insert(date, Entry{
            source: source,
            account: account,
            debet: None,
            kredit: Some(kredit),
        });
    }
}
