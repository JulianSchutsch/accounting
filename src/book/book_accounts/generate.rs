mod swedish;

use crate::book::book_accounts::BookAccounts;
use crate::book::book_result::*;
use crate::book::import::Import;

pub fn generate<'r, 'p: 'r>(import : &'p Import) -> BookResult<BookAccounts<'r>> {
    match import.settings.book_generator.as_str() {
        "swedish" => swedish::generate(import),
        _ => Err(BookError::new(format!("Book generator variant {} unknown", import.settings.book_generator)))
    }
}