pub mod income;

use super::types::*;
pub use income::Income;

#[derive(serde::Deserialize)]
pub enum Event {
    #[serde(rename="income")]
    Income(Income),
}

impl Event {
    pub fn date(&self) -> Date {
        match self {
            Event::Income(e) => e.date.clone(),
        }
    }
    
    pub fn id(&self) -> &String {
        match self {
            Event::Income(e) => &e.id,
        }
    }

    pub fn type_str(&self) -> &'static str {
        match self {
            Event::Income(_) => "income"
        }
    }
    
    pub fn describe(&self) -> String {
        match self {
            Event::Income(e) => format!("income {}, declared at {}, amount={}, moms={} currency={:?} method={:?}", e.id, e.date, e.amount.0, e.moms.0, e.currency, e.method)
        }
    }
}
