use crate::book::*;

mod swedish;

pub fn generate_using_settings(ledger: &mut Ledger, settings: &settings::Settings) -> BookResult {
    match settings.book_generator.as_str() {
        "swedish" => swedish::generate(ledger, settings),
        _ => Err(BookError::new(format!("Book generator variant {} unknown", settings.book_generator)))
    }
}