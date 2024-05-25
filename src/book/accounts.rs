use super::event::Event;
use super::types::*;

#[derive(Debug)]
pub struct AccountId(pub i32);

pub struct Entry<'l> {
    source: &'l Event,
    account: AccountId,
    amount: BookAmount,
}

type Entries<'l> = std::collections::BTreeMap<Date, Entry<'l>>;
type EntriesIter<'l, 's> = std::collections::btree_map::Iter<'s, Date, Entry<'l>>;

pub struct Accounts<'l> {
    entries: Entries<'l>,
}

impl<'s> Accounts<'s> {
    pub fn new<'l>() -> Accounts<'l> {
        Accounts{
            entries: Entries::new()
        }
    }
    pub fn iter(&self) -> EntriesIter { self.entries.iter() }   
    pub fn print(&self) {
        for (date, entry) in self.iter() {
            println!("Entry: {} {:?} {} pga={}", date, entry.account, entry.amount.0, entry.source.id());
        }
    }
    pub fn add_entry<'l: 's>(&mut self, date: &Date, source: &'l Event, account: AccountId, amount: BookAmount) {
        self.entries.insert(date.clone(), Entry{
            source: source,
            account: account,
            amount: amount
        });
    }
}
