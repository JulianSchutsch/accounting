use crate::book::*;

pub struct Second {
    pub book_accounts: BookAccounts,
}

impl Second {

    pub fn new(first: &phases::First, book: BookAccounts) -> Second {
        Self {
            book_accounts : book,
        }
    }

}