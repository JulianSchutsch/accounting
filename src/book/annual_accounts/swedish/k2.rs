use crate::book::book_result::*;
use crate::book::book_accounts::*;
use crate::book::book_accounts::book_account_id::BookAccountId;
use crate::book::import::Import;
use crate::book::settings;
use crate::book::types::*;

pub struct K2 {
    pub net_sales: Amount,
}

impl K2 {
    pub fn print(&self) {
        println!("Swedish-K2:");
        println!(" Net sales = {}", self.net_sales);
    }
}

fn calculate_net_sales(import: &Import, fiscal_year: DateRange, book_accounts: &BookAccounts) -> BookResult<Amount> {
    let filter = BookAccountsFilter::new(book_accounts, BookAccountIdRange::new(BookAccountId(3000), BookAccountId(3799))?, fiscal_year);
    let mut result = Amount(0.0);
    for (_, entry_list) in filter {
        for entry in entry_list {
            result = result + entry.amount.plain_amount();
        }
    }
    Ok(result)
}

pub fn generate(import: &Import, year: &settings::FiscalYear, book_accounts: &BookAccounts) -> BookResult<K2> {
    Ok(K2{
        net_sales:calculate_net_sales(import, year.fiscal_year, book_accounts)?
    })
}
