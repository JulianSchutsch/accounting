use crate::book::*;

pub struct Second {
    pub book: Book,
}

impl Second {

    pub fn new(first: &phases::First, book: Book) -> Second {
        Self { book }
    }

}