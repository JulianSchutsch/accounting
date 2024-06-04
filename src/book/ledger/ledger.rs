use std::collections::BTreeMap;
use crate::book::types::*;
use crate::book::ledger::*;
use crate::book::bookresult::*;

pub use event::{FiscalYearId, LedgerId};

type Events = std::collections::BTreeMap<LedgerId, Event>;
type EventsIter<'s> = std::collections::btree_map::Iter<'s, LedgerId, Event>;
type FiscalYears = std::collections::BTreeMap<FiscalYearId, FiscalYear>;

pub struct Ledger {
    pub events: Events,
    pub fiscal_years: FiscalYears,
}

impl Ledger {
    pub fn iter(&self) -> EventsIter { self.events.iter() }
    pub fn new() -> Ledger {
        Ledger{
            events: Events::new(),
            fiscal_years: BTreeMap::new(),
        }
    }

    pub fn print(&self) {
        for (_date, event) in self.iter() {
            println!("{:?}", event);
        }
    }
}
