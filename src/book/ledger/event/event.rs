use crate::book::Date;

use super::income::Income;
use super::invoice::Invoice;

#[derive(Debug, serde::Deserialize)]
pub enum Event {
    #[serde(rename="income")]
    Income(Income),
    #[serde(rename="invoice")]
    Invoice(Invoice)
}

impl Event {
    pub fn date(&self) -> Date {
        match self {
            Event::Income(e) => e.date,
            Event::Invoice(e) => e.date,
        }
    }

    pub fn id(&self) -> &String {
        match self {
            Event::Income(e) => &e.id,
            Event::Invoice(e) => &e.id,
        }
    }
}
