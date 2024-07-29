use crate::book::*;

#[derive(Clone)]
pub struct BookIdRange {
    begin: BookId,
    end: BookId
}

pub struct BookIdRangeIter {
    pos: BookId,
    end: BookId
}

impl std::iter::Iterator for BookIdRangeIter {
    type Item = BookId;
    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.pos<=self.end {
            Some(self.pos)
        } else {
            None
        };
        self.pos = BookId(self.pos.0+1);
        result
    }
}

impl BookIdRange {
    pub const FULL: BookIdRange = BookIdRange{ begin: BookId::MIN, end: BookId::MAX};

    pub fn iter(&self) -> BookIdRangeIter {
        BookIdRangeIter{ pos: self.begin, end: self.end }
    }

    pub const fn new(begin: BookId, end: BookId) -> Self {
        Self{ begin, end  }
    }

    pub const fn num_new(begin: i32, end: i32) -> Self {
        Self{ begin: BookId(begin), end: BookId(end) }
    }

    pub const fn single(id: BookId) -> Self {
        Self{ begin: id, end: id}
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