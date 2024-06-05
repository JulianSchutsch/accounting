use crate::book::ledger::LedgerId;
use crate::book::types::*;

use super::book_accounts::{BookAccounts, BookAccountEntriesIter};
use super::book_account_id_range::BookAccountIdRange;
use super::book_account_entry::BookAccountEntry;

pub struct BookAccountsFilter<'l1, 'l2: 'l1> {
    id_range: BookAccountIdRange,
    date_range: DateRange,
    book_iter: BookAccountEntriesIter<'l1, 'l2>
}

impl<'l1, 'l2> BookAccountsFilter<'l1, 'l2> {
    pub fn new<'p: 'l1>(book_accounts: &'p BookAccounts<'l2>, id_range: BookAccountIdRange, date_range: DateRange) -> Self {
        Self{id_range, date_range, book_iter: book_accounts.iter()}
    }
}

impl<'l1, 'l2> Iterator for BookAccountsFilter<'l1, 'l2>  {
    type Item = (LedgerId, Vec<&'l1 BookAccountEntry<'l2>>);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((entry_key, entry_list))=self.book_iter.next() {
            if self.date_range.contains(entry_key.date) {
                let result:Vec<&'l1 BookAccountEntry<'l2>> = entry_list.iter().filter(|e| self.id_range.contains(e.account)).collect();
                if !result.is_empty() {
                    return Some((entry_key.ledger_id, result));
                }
            }
        }
        None
    }
}