use crate::book::*;

#[derive(Clone)]
pub struct BookIdRange {
    begin: BookId,
    end: BookId
}

impl BookIdRange {
    pub const FULL: BookIdRange = BookIdRange{ begin: BookId::MIN, end: BookId::MAX};

    pub const fn new(begin: BookId, end: BookId) -> Self {
        Self{ begin, end  }
    }

    pub const fn num_new(begin: i32, end: i32) -> Self {
        Self{ begin: BookId(begin), end: BookId(end) }
    }

    pub fn contains(&self, id: BookId) -> bool {
        (id>=self.begin) && (id<=self.end)
    }
}

impl std::fmt::Display for BookIdRange{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.begin, self.end)
    }
}