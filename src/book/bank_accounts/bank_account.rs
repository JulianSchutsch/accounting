use std::collections::BTreeMap;

use crate::book::*;

pub struct BankPeriod {
    period: Period,
    filename: String
}

#[derive(Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub enum BankAccountType {
    #[serde(rename="account")]
    Account,
    #[serde(rename="private")]
    Private
}

type BankTransactions = BTreeMap<Date, Vec<BankTransaction>>;
pub type BankTransactionsIter<'l> = std::collections::btree_map::Iter<'l, Date, Vec<BankTransaction>>;

pub struct BankAccount {
    pub account_type: BankAccountType,
    pub initial_value: Amount,
    pub currency: Currency,
    pub references: BankAccountReferences,
    transactions: BankTransactions,
    periods: Vec<BankPeriod>,
}

impl BankAccount {
    pub fn description(&self) -> String {
        format!("{}", self.references)
    }

    pub fn iter_transactions(&self) -> BankTransactionsIter {
        self.transactions.iter()
    }

    pub fn new(account_type: BankAccountType, initial_value: Amount, currency: Currency, references: BankAccountReferences) -> Self {
        Self { account_type, initial_value, currency, references, transactions: BTreeMap::new(), periods: Vec::new() }
    }

    pub fn add_transaction(&mut self, date: Date, amount: Amount, references: BankTransactionReferences) -> BookResult {
        let transaction = BankTransaction::new(amount, references);
        self.transactions.entry(date).or_insert(vec![]).push(transaction);
        return Ok(());
    }

    pub fn add_period(&mut self,period: Period, filename: String) {
        self.periods.push(BankPeriod{ period, filename });
    }
}
