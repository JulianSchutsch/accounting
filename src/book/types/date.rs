use chrono::Datelike;

use crate::book::*;

const DATE_STRING_FORMAT: &'static str = "%Y-%m-%d";
type InternalDate = chrono::NaiveDate;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Date(InternalDate);


impl Date {
    pub fn id(&self) -> i32 {
        self.0.year()*10000 + (self.0.month() as i32)*100 + (self.0.day() as i32)
    }

    pub fn from_str(s: &str) -> BookResult<Date> {
        let naive = chrono::NaiveDate::parse_from_str(s, DATE_STRING_FORMAT).map_err(|e| BookError::new(format!("Failed to parse date {} with {}", s, e)))?;
        Ok(Date(naive))
    }

}

impl<'de> serde::Deserialize<'de> for Date {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Date, D::Error> {
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
