use crate::book::*;

mod swedish;

pub fn generate_fiscal_year(ledger: &mut Ledger, settings: &settings::Settings, fiscal_year: &settings::FiscalYear) -> BookResult {
    match settings.book_generator.as_str() {
        "swedish" => swedish::generate_fiscal_year(ledger, fiscal_year),
        _ => Err(BookError::new(format!("Book generator variant {} unknown", settings.book_generator)))
    }
}