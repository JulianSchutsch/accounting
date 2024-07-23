use crate::book::*;

#[derive(Clone)]
pub struct BookAccountIdRange {
    begin: BookId,
    end: BookId
}

impl BookAccountIdRange {
    pub const FULL: BookAccountIdRange = BookAccountIdRange{ begin: BookId::MIN, end: BookId::MAX};

    pub const fn new(begin: BookId, end: BookId) -> Self {
        Self{ begin, end  }
    }

    pub fn contains(&self, id: BookId) -> bool {
        (id>=self.begin) && (id<=self.end)
    }
}

impl std::fmt::Display for BookAccountIdRange{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.begin, self.end)
    }
}