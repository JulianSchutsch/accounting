use crate::book::*;

#[derive(Clone, PartialEq)]
pub enum BookAccountSide {
    Debit,
    Credit,
    Both
}

impl BookAccountSide {
    pub fn contains(&self, other: BookAccountAmount) -> bool {
        (*self==BookAccountSide::Both) || match other {
            BookAccountAmount::Debit(_) => *self==BookAccountSide::Debit,
            BookAccountAmount::Credit(_) => *self==BookAccountSide::Credit
        }
    }
}

#[derive(Clone)]
pub struct BookAccountsFilter<'l1, 'l2: 'l1> {
    id_range: BookAccountIdRange,
    date_range: DateRange,
    side: BookAccountSide,
    book_iter: BookAccountEntriesIter<'l1, 'l2>
}

pub struct BookAccountsFilterBuilder {
    id_range: BookAccountIdRange,
    date_range: DateRange,
    side: BookAccountSide,
}

impl BookAccountsFilterBuilder {
    pub fn new() -> Self {
        Self{
            id_range: BookAccountIdRange::FULL,
            date_range: DateRange::FULL,
            side: BookAccountSide::Both
        }
    }
    pub fn limit_id(self, id_range: BookAccountIdRange) -> Self {
        Self{id_range, date_range: self.date_range, side: self.side}
    }

    pub fn limit_side(self, side: BookAccountSide) -> Self {
        Self{id_range: self.id_range, date_range: self.date_range, side}
    }

    pub fn limit_date(self, date_range: DateRange) -> Self {
        Self{id_range: self.id_range, date_range, side: self.side}
    }

    pub fn iter<'p, 'l>(self, book_accounts: &'p BookAccounts<'l>) -> BookAccountsFilter<'p, 'l> {
        BookAccountsFilter {
            id_range: self.id_range,
            date_range: self.date_range,
            side: self.side,
            book_iter: book_accounts.iter()
        }
    }
}

impl<'l1, 'l2> Iterator for BookAccountsFilter<'l1, 'l2>  {
    type Item = (LedgerId, Vec<&'l1 BookAccountEntry<'l2>>);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((entry_key, entry_list))=self.book_iter.next() {
            if self.date_range.contains(entry_key.date) {
                let result:Vec<&'l1 BookAccountEntry<'l2>> = entry_list.iter().filter(|e| self.id_range.contains(e.account) && self.side.contains(e.amount)).collect();
                if !result.is_empty() {
                    return Some((entry_key.ledger_id, result));
                }
            }
        }
        None
    }
}