use crate::book::*;

use super::result::*;
use super::balance::*;

pub struct K2 {
    pub result: Result,
    pub balance: Balance
}

impl K2 {
    pub fn generate(import: &phases::First, year: &settings::FiscalYear, book: &Book) -> BookResult<K2> {
        Ok(K2 {
            result: Result::generate(year.fiscal_year, book)?,
            balance: Balance::generate(year.fiscal_year, book)?
        })
    }
}
