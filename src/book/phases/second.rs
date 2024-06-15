use crate::book::*;

pub struct Second<'l1> {
    pub book_accounts: BookAccounts<'l1>,
    pub consumed_bank_transactions: BankTransactionConsumer<'l1>
}

impl<'l1> Second<'l1> {

    pub fn new(first: &phases::First) -> Second<'l1> {
        Self {
            book_accounts : BookAccounts::new(first.settings.book_currency),
            consumed_bank_transactions : BankTransactionConsumer::new()
        }
    }

}