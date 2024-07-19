use crate::book::*;

pub struct BankTransactionsFilterBuilder {
    date_range: Period,
}

type BankTransactionVecIter<'l> = std::slice::Iter<'l, BankTransaction>;

pub struct BankTransactionsFilter<'l1> {
    date_range: Period,
    bank_account: &'l1 BankAccount,
    iterator: BankTransactionsIter<'l1>,
    within_day_iterator: BankTransactionVecIter<'l1>
}

impl BankTransactionsFilterBuilder {
    pub fn new() -> Self {
        Self { date_range: Period::FULL }
    }
}

impl BankTransactionsFilterBuilder {
    pub fn iter<'l1>(&self, bank_account: &'l1 BankAccount) -> BankTransactionsFilter<'l1> {
        BankTransactionsFilter {
            date_range: self.date_range,
            bank_account,
            iterator: bank_account.iter_transactions(),
            within_day_iterator: BankTransactionVecIter::default()
        }
    }
}

impl<'l1> Iterator for BankTransactionsFilter<'l1> {
    type Item = &'l1 BankTransaction;
    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            if let Some(transaction) = self.within_day_iterator.next() {
                return Some(transaction);
            }
            while let Some((date, next_vec)) = self.iterator.next() {
                if self.date_range.contains(*date) {
                    self.within_day_iterator = next_vec.iter();
                    continue 'outer
                }
            }
            return None
        }
    }
}