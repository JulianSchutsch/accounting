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
    #[serde(rename="privat")]
    Privat
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

    pub fn consume_transaction<'c, 's: 'c>(&'s self, consumer: &mut BankTransactionConsumer<'c>, date: Date, amount: Amount, references: &BankTransactionReferences) -> BookResult {
        if let Some(transactions)=self.transactions.get(&date) {
            for entry in transactions.iter() {
                if entry.is_match(amount, references) {
                    if !consumer.try_consume(entry) {
                        return Err(BookError::new(format!("Matching transaction for amount={} and references={:?} already consumed", amount, references)));
                    }
                    return Ok(());
                }
            }
        }
        Err(BookError::new(format!("No matching transaction for amount={} and references={:?} found", amount, references)))
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
