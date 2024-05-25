pub use serde::{Deserialize, Deserializer};

type InternalDate = chrono::NaiveDate;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Date(InternalDate);

const DateStringFormat: &'static str = "%Y-%m-%d";

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Date, D::Error> {
        let string = String::deserialize(deserializer)?;
        let naive = chrono::NaiveDate::parse_from_str(&string, DateStringFormat).map_err(serde::de::Error::custom)?;
        Ok(Date(naive))
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}
