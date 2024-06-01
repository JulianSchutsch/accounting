use crate::book::ledger::*;
use crate::book::types::*;

use super::accountid::AccountId;

#[derive(Clone, Copy)]
pub struct Entry<'l> {
    source: &'l Event,
    account: AccountId,
    debet: Option<BookAmount>,
    kredit: Option<BookAmount>,
}

type EntryKey = (Date, LedgerId);
type Entries<'l> = std::collections::BTreeMap<EntryKey, Vec<Entry<'l>>>;
type EntriesIter<'l, 's> = std::collections::btree_map::Iter<'s, EntryKey, Vec<Entry<'l>>>;

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
                println!("Entry: {:?} {} {:?} {:?} {:?} pga={}", ledger_id, date, entry.account, entry.debet, entry.kredit, entry.source.id());
            }
        }
    }

    pub fn add_entry<'l: 's>(&mut self, ledger_id: LedgerId, source: &'l Event,  account: AccountId, debet: Option<BookAmount>, kredit: Option<BookAmount>) {
        let entry = Entry{
            source: source,
            account: account,
            debet: debet,
            kredit: kredit,
        };
        self.entries.entry((source.date(), ledger_id)).and_modify(|e| e.push(entry)).or_insert(vec![entry]);
    }

    pub fn add_entry_debet<'l: 's>(&mut self, ledger_id: LedgerId, source: &'l Event, account: AccountId, debet: BookAmount) {
        self.add_entry(ledger_id, source, account, Some(debet), None);
    }

    pub fn add_entry_kredit<'l: 's>(&mut self, ledger_id: LedgerId, source: &'l Event, account: AccountId, kredit: BookAmount) {
        self.add_entry(ledger_id, source, account, None, Some(kredit));
    }
}
