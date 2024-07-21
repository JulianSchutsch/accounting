use crate::book::*;

use super::active_associables::*;

pub struct Params<'l1> {
    pub first: &'l1 phases::First,
    pub book: BookAccounts,
}

impl<'l1> Params<'l1> {
    pub fn new(first: &'l1 phases::First) -> Self {
        Self { first, book: BookAccounts::new(first.settings.book_currency) }
    }
}