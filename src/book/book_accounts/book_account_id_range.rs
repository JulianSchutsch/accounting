use super::book_account_id::BookAccountId;
use crate::book::book_result::*;

pub struct BookAccountIdRange {
    begin: BookAccountId,
    end: BookAccountId
}

impl BookAccountIdRange {
    pub fn new(begin: BookAccountId, end: BookAccountId) -> BookResult<Self> {
        if begin<=end {
            return Ok(Self{ begin, end  });
        }
        Err(BookError::new(format!("Invalid BookAccountRange: {} {}", begin, end)))
    }
    pub fn contains(&self, id: BookAccountId) -> bool {
        (id>=self.begin) && (id<=self.end)
    }
}