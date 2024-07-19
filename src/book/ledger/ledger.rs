use std::collections::BTreeMap;

use crate::book::*;

type Events = BTreeMap<LedgerId, Event>;
type EventsIter<'s> = std::collections::btree_map::Iter<'s, LedgerId, Event>;
type FiscalYears = BTreeMap<FiscalYearId, Period>;

pub struct Ledger {
    pub events: Events,
    pub fiscal_years: FiscalYears,
    pub ledger_id: LedgerIdGenerator,
}

impl Ledger {
    pub fn iter(&self) -> EventsIter { self.events.iter() }
    pub fn new() -> Ledger {
        Ledger{
            events: Events::new(),
            fiscal_years: BTreeMap::new(),
            ledger_id: LedgerIdGenerator::new()
        }
    }

    pub fn select_fiscal_year(&mut self, period: Period) {
        self.ledger_id.select_fiscal_year(period);
    }

    pub fn get_mut_invoice_by_id(&mut self, id: &String) -> Option<&mut Invoice> {
        for (_, event) in self.events.iter_mut() {
            if event.id()==id {
                match event {
                    Event::Invoice(invoice) => return Some(invoice),
                    _ => ()
                }
            }
        }
        None
    }

    pub fn print(&self) {
        for (_date, event) in self.iter() {
            println!("{:?}", event);
        }
    }
}
