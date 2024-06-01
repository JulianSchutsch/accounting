use crate::book::types::*;
use crate::book::ledger::*;
use crate::book::bookresult::*;

use super::event::LedgerId;

type Events = std::collections::BTreeMap<LedgerId, Event>;
type EventsIter<'s> = std::collections::btree_map::Iter<'s, LedgerId, Event>;

pub struct Ledger {
    events: Events,
    next_id: LedgerId,
}

impl Ledger {
    pub fn iter(&self) -> EventsIter { self.events.iter() }
    pub fn new() -> Ledger {
        Ledger{
            events: Events::new(),
            next_id: LedgerId(0)
        }
    }

    pub fn add_from_file(&mut self, path: String) -> BookResult<()> {
        let file = std::fs::File::open(path).map_err(|_| "Failed to open path")?;
        let reader = std::io::BufReader::new(file);
        let entries: Vec<Event> = serde_yaml::from_reader(reader)?;
        for entry in entries {
            self.events.insert(self.next_id, entry);
            self.next_id = LedgerId(self.next_id.0+1);
        }
        Ok(())
    }

    pub fn print(&self) {
        for (_date, event) in self.iter() {
            println!("{:?}", event);
        }
    }
}
