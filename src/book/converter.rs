use crate::book::bookresult::BookResult;
use super::types::*;

pub trait Converter {
    fn book_currency(&self) -> Currency;
    fn amount_into_book(&self, date: Date, currency: Currency, amount: Amount) -> BookResult<BookAmount>;
    fn moms_into_book(&self, date: Date, currency: Currency, amount: Moms) -> BookResult<BookAmount>;
}
