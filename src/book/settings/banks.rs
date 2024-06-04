pub mod csv;

use crate::book::bookresult::BookResult;

pub use csv::CSV;

#[derive(serde::Deserialize)]
pub enum PlainBanks {
    #[serde(rename="csv")]
    CSV(csv::PlainCSV)
}

pub enum Banks {
    CSV(CSV)
}

impl PlainBanks {
    pub fn to_banks(&self) -> BookResult<Banks> {
        match self {
            PlainBanks::CSV(e) => {
                 Ok(Banks::CSV(e.to_csv()?))
            }
        }
    }
}