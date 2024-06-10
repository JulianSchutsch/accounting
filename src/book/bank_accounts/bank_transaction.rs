use crate::book::*;

pub struct BankTransaction {
    amount: Amount,
    references: BankTransactionReferences,
}

impl BankTransaction {
    pub fn new(amount: Amount, references: BankTransactionReferences) -> Self {
        Self { amount, references }
    }

    pub fn is_match(&self, amount: Amount, references: &BankTransactionReferences) -> bool {
        self.amount==amount && references.is_match(&self.references)
    }
}