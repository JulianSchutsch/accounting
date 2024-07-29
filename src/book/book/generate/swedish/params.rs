use crate::book::*;

pub struct Params<'l1> {
    pub first: &'l1 phases::First,
    pub book: Book,
}

impl<'l1> Params<'l1> {
    pub fn new(first: &'l1 phases::First) -> Self {
        Self { first, book: Book::new(first.settings.book_currency) }
    }
}