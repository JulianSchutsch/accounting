use std::collections::BTreeMap;

use crate::book::*;

type AssociablePtr<TA> = *const dyn Associable<TA>;

pub struct Associables<TA> {
    entries: BTreeMap<AssociablePtr<TA>, AssociableBox<TA>>
}

impl<TA> Associables<TA> {
    pub fn new() -> Self {
        Self{ entries: BTreeMap::new() }
    }

    pub fn register(&mut self, associable: AssociableBox<TA>) {
        self.entries.insert(&*associable, associable);
    }

    pub fn associate(&mut self, data: &TA) -> BookResult<bool> {
        let mut result: bool = false;
        let mut remove_opt : Option<AssociablePtr<TA>> = None;
        for (addr, entry) in self.entries.iter_mut() {
            match entry.associate(data)? {
                AssociableChange::Match => { result=true; break; },
                AssociableChange::Close => { result=true; remove_opt=Some(*addr); break; }
                AssociableChange::NoMatch => ()
            }
        }
        if let Some(remove) = remove_opt {
            self.entries.remove(&remove);
        }
        Ok(result)
    }
}