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
    pub values: BTreeMap<Date, Amount>
}

impl BankAccount {
    pub fn description(&self) -> String {
        format!("{}", self.references)
    }

    pub fn new(account_type: BankAccountType, initial_value: Amount, currency: Currency, references: BankAccountReferences) -> Self {
        Self { account_type, initial_value, currency, references, periods: Vec::new(), values: BTreeMap::new() }
    }

    pub fn add_period(&mut self, period: Period, filename: String) {
        self.periods.push(BankPeriod{ period, filename });
    }

    pub fn latest_value(&self, upper_date: Date) -> Option<Amount> {
        let mut result: Option<Amount> = None;
        for (&date, &value) in self.values.iter() {
            if date>upper_date {
                return result;
            }
            result = Some(value);
        }
        return result;
    }

    pub fn add_value(&mut self, date: Date, value: Amount) {
        *self.values.entry(date).or_insert(value) = value;
    }
}
