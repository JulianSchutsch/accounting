pub mod csv;
pub mod skv;

use crate::book::*;

pub use csv::CSV;
pub use skv::SKV;

#[derive(serde::Deserialize)]
pub enum PlainBanks {
    #[serde(rename="csv")]
    CSV(csv::PlainCSV),
    #[serde(rename="skv")]
    SKV(skv::PlainSKV)
}

pub enum Banks {
    CSV(CSV),
    SKV(SKV)
}

impl PlainBanks {
    pub fn to_banks(&self) -> BookResult<Banks> {
        match self {
            PlainBanks::CSV(e) => {
                 Ok(Banks::CSV(e.to_csv()?))
            },
            PlainBanks::SKV(e) => {
                Ok(Banks::SKV(e.to_skv()?))
            }
        }
    }
}