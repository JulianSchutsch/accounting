use crate::book::*;

pub fn all_transactions_consumed(bank_accounts: &BankAccounts, consumed: &BankTransactionConsumer) -> bool {
    bank_accounts.iter().all(|(_, account)| {
        account.iter_transactions().all(|(_, vec)| {
            vec.iter().all(|e| consumed.is_consumed(e))
        })
    })
}