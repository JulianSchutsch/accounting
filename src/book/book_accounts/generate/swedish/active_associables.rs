use crate::book::*;

use super::params::*;

pub type TransactionAssociables<'l1> = Associables<Transaction, Params<'l1>>;
pub type ExchangeAssociables<'l1> = Associables<Exchange, Params<'l1>>;

pub struct ActiveAssociables<'l1> {
    pub transactions: TransactionAssociables<'l1>,
    pub exchanges: ExchangeAssociables<'l1>
}

impl ActiveAssociables<'_> {
    pub fn new() -> Self {
        Self {
            transactions: Associables::new(),
            exchanges: ExchangeAssociables::new()
        }
    }
}