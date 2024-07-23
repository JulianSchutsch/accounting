use crate::book::*;

#[derive(Clone, PartialEq)]
pub enum BookSideFilter {
    Debit,
    Credit,
    Both
}

impl BookSideFilter {
    pub fn contains(&self, other: BookAmount) -> bool {
        (*self==BookSideFilter::Both) || match other {
            BookAmount::Debit(_) => *self==BookSideFilter::Debit,
            BookAmount::Credit(_) => *self==BookSideFilter::Credit

        }
    }
}

#[derive(Clone)]
pub struct BookFilter<'l1> {
    id_range: BookIdRange,
    date_range: Period,
    side: BookSideFilter,
    book_iter: BookAccountEntriesIter<'l1>
}

pub struct BookFilterBuilder {
    id_range: BookIdRange,
    date_range: Period,
    side: BookSideFilter,
}

impl BookFilterBuilder {
    pub fn new() -> Self {
        Self{
            id_range: BookIdRange::FULL,
            date_range: Period::FULL,
            side: BookSideFilter::Both
        }
    }
    pub fn limit_id(self, id_range: BookIdRange) -> Self {
        Self{id_range, date_range: self.date_range, side: self.side}
    }

    pub fn limit_side(self, side: BookSideFilter) -> Self {
        Self{id_range: self.id_range, date_range: self.date_range, side}
    }

    pub fn limit_date(self, date_range: Period) -> Self {
        Self{id_range: self.id_range, date_range, side: self.side}
    }

    pub fn build(self, book_accounts: &Book) -> BookFilter {
        BookFilter {
            id_range: self.id_range,
            date_range: self.date_range,
            side: self.side,
            book_iter: book_accounts.iter()
        }
    }
}

impl<'l1> Iterator for BookFilter<'l1>  {
    type Item = ((Date, LedgerId), Vec<&'l1 BookEntry>);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((entry_key, entry_list))=self.book_iter.next() {
            if self.date_range.contains(entry_key.date) {
                let result:Vec<&'l1 BookEntry> = entry_list.iter().filter(|e| self.id_range.contains(e.account) && self.side.contains(e.amount)).collect();
                if !result.is_empty() {
                    return Some(((entry_key.date, entry_key.ledger_id), result));
                }
            }
        }
        None
    }
}