use std::collections::BTreeMap;

use crate::book::*;

type Events = BTreeMap<LedgerId, Event>;
type EventsIter<'s> = std::collections::btree_map::Iter<'s, LedgerId, Event>;
type FiscalYears = BTreeMap<FiscalYearId, Period>;

pub struct Ledger {
    pub book_currency: Currency,
    pub events: Events,
}

impl Ledger {
    pub fn iter(&self) -> EventsIter { self.events.iter() }
    pub fn new(book_currency: Currency) -> Ledger {
        Ledger{
            book_currency,
            events: Events::new()
        }
    }

    pub fn print(&self) {
        for (ledger_id, event) in self.iter() {
            println!("{} {:?}", ledger_id, event);
        }
    }
}
