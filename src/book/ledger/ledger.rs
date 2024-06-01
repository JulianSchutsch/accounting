use crate::book::types::*;
use crate::book::ledger::Event;

type Events = std::collections::BTreeMap<Date, Event>;
type EventsIter<'s> = std::collections::btree_map::Iter<'s, Date, Event>;

pub struct Ledger {
    events: Events,
}

impl Ledger {
    pub fn iter(&self) -> EventsIter { self.events.iter() }
    pub fn new() -> Ledger {
        Ledger{events: Events::new()}
    }

    pub fn add_from_file(&mut self, path: String) -> Result<(), String> {
        let file = std::fs::File::open(path).map_err(|_| "Failed to open path")?;
        let reader = std::io::BufReader::new(file);
        let entries: Vec<Event> = serde_yaml::from_reader(reader).unwrap();
        for entry in entries {
            self.events.insert(entry.date(), entry);
        }
        Ok(())
    }

    pub fn print(&self) {
        for (_date, event) in self.iter() {
            println!("{:?}", event);
        }
    }
}
