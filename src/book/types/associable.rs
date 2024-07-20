use crate::book::*;

pub enum AssociableChange {
    NoMatch,
    Match,
    Close
}

pub trait Associable<TA> {
    fn associate(&mut self, data: &TA) -> BookResult<AssociableChange>;
}

pub type AssociableBox<TA> = Box<dyn Associable<TA>>;
