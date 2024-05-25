use super::event::Event;

pub struct Entry<'l> {
    source: &'l Event,
    account: i32,
    amount: i64,
}

type Entries<'l> = std::collections::BTreeMap<time::OffsetDateTime, Entry<'l>>;
type EntriesIter<'l, 's> = std::collections::btree_map::Iter<'s, time::OffsetDateTime, Entry<'l>>;

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
            println!("Entry: {} {} {} pga={}", date, entry.account, entry.amount, entry.source.id());
        }
    }
    pub fn add_entry<'l: 's>(&mut self, date: time::OffsetDateTime, source: &'l Event, account:i32, amount: i64) {
        self.entries.insert(date, Entry{
            source: source,
            account: account,
            amount: amount
        });
    }
}
