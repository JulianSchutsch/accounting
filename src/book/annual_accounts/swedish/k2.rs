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

fn calculate_net_sales(import: &phases::First, fiscal_year: Period, book: &Book) -> BookResult<Amount> {
    book::tools::period_sum(book, fiscal_year, BookIdRange::new(BookId(3000), BookId(3799)), BookSideFilter::Both)
}

pub fn generate(import: &phases::First, year: &settings::FiscalYear, book: &Book) -> BookResult<K2> {
    Ok(K2{
        net_sales:calculate_net_sales(import, year.fiscal_year, book)?
    })
}
