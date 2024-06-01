pub use serde::{Deserialize, Deserializer};

type InternalDate = chrono::NaiveDate;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Date(InternalDate);

const DATE_STRING_FORMAT: &'static str = "%Y-%m-%d";

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Date, D::Error> {
        let string = String::deserialize(deserializer)?;
        let naive = chrono::NaiveDate::parse_from_str(&string, DATE_STRING_FORMAT).map_err(serde::de::Error::custom)?;
        Ok(Date(naive))
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}
