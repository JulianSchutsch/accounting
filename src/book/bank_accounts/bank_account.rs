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

pub struct BankAccount {
    pub account_type: BankAccountType,
    pub initial_value: Amount,
    pub currency: Currency,
    pub references: BankAccountReferences,
    periods: Vec<BankPeriod>,
}

impl BankAccount {
    pub fn description(&self) -> String {
        format!("{}", self.references)
    }

    pub fn new(account_type: BankAccountType, initial_value: Amount, currency: Currency, references: BankAccountReferences) -> Self {
        Self { account_type, initial_value, currency, references, periods: Vec::new() }
    }

    pub fn add_period(&mut self,period: Period, filename: String) {
        self.periods.push(BankPeriod{ period, filename });
    }
}
