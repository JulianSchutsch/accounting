mod swedish;

use crate::book::*;

pub fn generate<'r, 'p: 'r>(import : &'p phases::First) -> BookResult<phases::Second<'r>> {
    match import.settings.book_generator.as_str() {
        "swedish" => swedish::generate(import).map_err(|e| e.extend("Failed to generate swedish book")),
        _ => Err(BookError::new(format!("Book generator variant {} unknown", import.settings.book_generator)))
    }
}