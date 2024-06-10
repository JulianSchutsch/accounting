use crate::book::*;

pub fn verify(book_accounts: &BookAccounts) -> BookResult {
    for (entry_key, entry_list) in book_accounts.iter() {
        let mut sum = BookAccountSum::zero();
        for entry in entry_list.iter() {
            sum += entry.amount;
        }
        if !sum.valid() {
            return Err(BookError::new(format!("Book balance invalid starting with {}", entry_key.date)));
        }
    }
    Ok(())
}