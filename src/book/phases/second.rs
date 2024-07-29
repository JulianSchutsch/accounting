use crate::book::*;

pub struct Second {
    pub book: Book,
}

impl Second {

    pub fn new(_first: &phases::First, book: Book) -> Second {
        Self { book }
    }

}