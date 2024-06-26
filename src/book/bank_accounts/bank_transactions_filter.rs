use crate::book::*;

pub struct BankTransactionsFilterBuilder {
    date_range: DateRange
}

type BankTransactionVecIter<'l> = std::slice::Iter<'l, BankTransaction>;

pub struct BankTransactionsFilter<'l1, 'l2, 'l2_l> {
    date_range: DateRange,
    bank_account: &'l1 BankAccount,
    consumed: Option<&'l2 BankTransactionConsumer<'l2_l>>,
    iterator: BankTransactionsIter<'l1>,
    within_day_iterator: BankTransactionVecIter<'l1>
}

impl BankTransactionsFilterBuilder {
    pub fn new() -> Self {
        Self { date_range: DateRange::FULL }
    }
}

impl BankTransactionsFilterBuilder {
    pub fn iter<'l2, 'l2_1, 'l1:'l2_1>(&self, bank_account: &'l1 BankAccount, consumed: Option<&'l2 BankTransactionConsumer<'l2_1>>) -> BankTransactionsFilter<'l1, 'l2, 'l2_1> {
        BankTransactionsFilter {
            date_range: self.date_range,
            bank_account,
            consumed,
            iterator: bank_account.iter_transactions(),
            within_day_iterator: BankTransactionVecIter::default()
        }
    }
}

impl<'l1, 'l2, 'l2_1> Iterator for BankTransactionsFilter<'l1, 'l2, 'l2_1> {
    type Item = &'l1 BankTransaction;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(transaction) = self.within_day_iterator.next() {
                if self.consumed.is_some_and(|c| c.is_consumed(transaction)) {
                    return Some(transaction);
                }
            }
            while let Some((date, next_vec)) = self.iterator.next() {
                if self.date_range.contains(*date) {
                    self.within_day_iterator = next_vec.iter();
                    continue
                }
            }
            return None
        }
    }
}