use crate::book::*;

pub type TransactionAssociables = Associables<Transaction>;

pub struct ActiveAssociables {
    pub transactions: TransactionAssociables
}

impl ActiveAssociables {
    pub fn new() -> Self {
        Self {
            transactions: Associables::new()
        }
    }
}