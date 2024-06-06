use crate::book::{BookResult, Date};

use super::income::Income;
use super::invoice::Invoice;
use super::payment::Payment;

#[derive(Debug, serde::Deserialize)]
pub enum Event {
    #[serde(rename="income")]
    Income(Income),
    #[serde(rename="invoice")]
    Invoice(Invoice),
    #[serde(rename="payment")]
    Payment(Payment)
}

impl Event {
    pub fn date(&self) -> Date {
        match self {
            Event::Income(e) => e.date,
            Event::Invoice(e) => e.date,
            Event::Payment(e) => e.date
        }
    }

    pub fn id(&self) -> &String {
        match self {
            Event::Income(e) => &e.id,
            Event::Invoice(e) => &e.id,
            Event::Payment(e) => &e.id
        }
    }

    pub fn verify(&self) -> BookResult<> {
        match self {
            Event::Income(e) => e.verify(),
            Event::Invoice(e) => e.verify(),
            Event::Payment(e) => e.verify()
        }
    }
}
