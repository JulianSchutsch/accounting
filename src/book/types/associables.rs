use std::collections::BTreeMap;

use crate::book::*;

type AssociablePtr<TA, TH> = *const dyn Associable<TA, TH>;

pub struct Associables<TA, TH> {
    entries: BTreeMap<AssociablePtr<TA, TH>, AssociableBox<TA, TH>>
}

impl<TA, TH> Associables<TA, TH> {
    pub fn new() -> Self {
        Self{ entries: BTreeMap::new() }
    }

    pub fn register(&mut self, associable: AssociableBox<TA, TH>) {
        self.entries.insert(&*associable, associable);
    }

    pub fn associate(& mut self, ledger_id: LedgerId, data: &TA, help: &mut TH) -> BookResult<bool> {
        let mut result: bool = false;
        let mut remove_opt : Option<AssociablePtr<TA, TH>> = None;
        for (addr, entry) in self.entries.iter_mut() {
            match entry.associate(ledger_id, data, help)? {
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