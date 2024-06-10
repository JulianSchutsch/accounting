use crate::book::*;

#[derive(Clone)]
pub struct BookAccountIdRange {
    begin: BookAccountId,
    end: BookAccountId
}

impl BookAccountIdRange {
    pub const FULL: BookAccountIdRange = BookAccountIdRange{ begin: BookAccountId::MIN, end: BookAccountId::MAX};

    pub fn new(begin: BookAccountId, end: BookAccountId) -> Self {
        Self{ begin, end  }
    }

    pub fn contains(&self, id: BookAccountId) -> bool {
        (id>=self.begin) && (id<=self.end)
    }
}