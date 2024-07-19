use crate::book::*;

pub enum AssociableChange {
    NoMatch,
    Match,
    Close
}

pub trait Associable<TA, TC> {
    fn associate(&mut self, data: &TA) -> BookResult<AssociableChange>;
    fn close(&mut self, data: &TC) -> BookResult<bool>;
}

pub type AssociableBox<TA, TC> = Box<dyn Associable<TA, TC>>;