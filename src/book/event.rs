pub mod income;

pub use income::Income;

#[derive(serde::Deserialize)]
pub enum Event {
    #[serde(rename="income")]
    Income(Income),
}

impl Event {
    pub fn date(&self) -> time::OffsetDateTime {
        match self {
            Event::Income(e) => e.date,
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
            Event::Income(e) => format!("income {}, declared at {}, amount={} {}, moms={} currency={} method={:?}", e.id, e.date, e.amount, e.currency, e.moms, e.currency, e.method)
        }
    }
}
