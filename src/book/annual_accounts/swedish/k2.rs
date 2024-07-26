use crate::book::*;

use super::result::*;
use super::balance::*;

#[derive(Default)]
pub struct K2 {
    pub result: Result,
    pub balance: Balance
}

impl K2 {
    pub fn new() -> Self {
        Self{..Default::default()}
    }

    pub fn generate(import: &phases::First, year: &settings::FiscalYear, book: &Book, previous: &K2) -> BookResult<K2> {
        let result = Result::generate(year.fiscal_year, book)?;
        let balance = Balance::generate(year.fiscal_year, book, &previous.balance, &result)?;
        Ok(K2 { result, balance })
    }
}
