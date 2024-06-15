use crate::book::*;

pub struct K2 {
    pub net_sales: Amount,
}

impl K2 {
    pub fn print(&self) {
        println!("Swedish-K2:");
        println!(" Net sales = {}", self.net_sales);
    }
}

fn calculate_net_sales(import: &phases::First, fiscal_year: DateRange, book_accounts: &BookAccounts) -> BookResult<Amount> {
    let filter = BookAccountsFilterBuilder::new()
        .limit_id(BookAccountIdRange::new(BookAccountId(3000), BookAccountId(3799)))
        .limit_date(fiscal_year).iter(book_accounts);
    let mut result = Amount(0.0);
    for (_, entry_list) in filter {
        for entry in entry_list {
            result = result + entry.amount.plain_amount();
        }
    }
    Ok(result)
}

pub fn generate(import: &phases::First, year: &settings::FiscalYear, book_accounts: &BookAccounts) -> BookResult<K2> {
    Ok(K2{
        net_sales:calculate_net_sales(import, year.fiscal_year, book_accounts)?
    })
}
