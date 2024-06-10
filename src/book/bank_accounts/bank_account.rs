use std::collections::BTreeMap;

use crate::book::*;

#[derive(Clone)]
pub struct BankTransaction {
    amount: Amount,
    references: Vec<String>
}

pub struct BankPeriod {
    period: Period,
    filename: String
}

#[derive(Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub enum BankAccountType {
    #[serde(rename="account")]
    Account,
    #[serde(rename="privat")]
    Privat
}

pub struct BankAccount {
    pub account_type: BankAccountType,
    initial_value: Amount,
    currency: Currency,
    pub references: BankAccountReferences,
    transactions: BTreeMap<Date, Vec<BankTransaction>>,
    periods: Vec<BankPeriod>,
}

impl BankAccount {
    pub fn new(account_type: BankAccountType, initial_value: Amount, currency: Currency, references: BankAccountReferences) -> Self {
        Self { account_type, initial_value, currency, references, transactions: BTreeMap::new(), periods: Vec::new() }
    }

    pub fn add_transaction(&mut self, date: Date, amount: Amount, references: Vec<String>) -> BookResult {
        let transaction = BankTransaction{amount, references};
        self.transactions.entry(date).or_insert(vec![]).push(transaction);
        return Ok(());
    }

    pub fn add_period(&mut self,period: Period, filename: String) {
        self.periods.push(BankPeriod{ period, filename });
    }
}
