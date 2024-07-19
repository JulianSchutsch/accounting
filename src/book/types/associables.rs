use std::collections::BTreeMap;

use crate::book::*;

type AssociablePtr<TA,TC> = *const dyn Associable<TA,TC>;

pub struct Associables<TA,TC> {
    entries: BTreeMap<AssociablePtr<TA, TC>, AssociableBox<TA, TC>>
}

impl<TA, TC> Associables<TA, TC> {
    pub fn new() -> Self {
        Self{ entries: BTreeMap::new() }
    }

    pub fn register(&mut self, associable: AssociableBox<TA, TC>) {
        self.entries.insert(&*associable, associable);
    }

    pub fn associate(&mut self, data: &TA) -> BookResult<bool> {
        let mut result: bool = false;
        let mut remove_opt : Option<AssociablePtr<TA, TC>> = None;
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

    pub fn close(&mut self, data: &TC) -> BookResult<bool> {
        let mut remove_opt : Option<AssociablePtr<TA, TC>> = None;
        for (addr, entry) in self.entries.iter_mut() {
            if entry.close(data)? {
                remove_opt = Some(*addr);
                break;
            }
        }
        if let Some(remove) = remove_opt {
            self.entries.remove(&remove);
            return Ok(true);
        }
        Ok(false)

    }
}