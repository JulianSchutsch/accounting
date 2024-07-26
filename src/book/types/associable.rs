use crate::book::*;

pub enum AssociableChange {
    NoMatch,
    Match,
    Close
}

pub trait Associable<TA, TH> {
    fn associate(&mut self, ledger_id: LedgerId, data: &TA, help: &mut TH) -> BookResult<AssociableChange>;
    fn describe(&self) -> String;
}

pub type AssociableBox<TA, TH> = Box<dyn Associable<TA, TH>>;
