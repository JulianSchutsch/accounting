use crate::book::*;

use super::annual_accounts::AnnualAccounts;
use super::swedish;

fn generate_year(import: &phases::First, year: &settings::FiscalYear, book_accounts: &BookAccounts) -> BookResult<AnnualAccounts> {
    match year.annual_accounts_method {
        settings::AnnualAccountsMethod::SwedishK2 => Ok(AnnualAccounts::SwedishK2(swedish::k2::generate(import, year, book_accounts)?))
    }
}

pub fn generate(import: &phases::First, book_accounts: &BookAccounts, year: Period) -> BookResult<AnnualAccounts> {
    for f_year in import.settings.fiscal_years.iter() {
        if f_year.fiscal_year == year {
            return generate_year(import, &f_year, book_accounts)
        }
    }
    Err(BookError::new(format!("Fiscal year {} not found in the settings.", year)))
}