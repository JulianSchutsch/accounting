use chrono::Datelike;

use crate::book::*;

const DATE_STRING_FORMAT: &'static str = "%Y-%m-%d";
type InternalDate = chrono::NaiveDate;

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Date(InternalDate);

impl Date {
    pub const MIN: Date = Date(InternalDate::MIN);
    pub const MAX: Date = Date(InternalDate::MAX);

    pub fn new(year: i32, month: u32, day: u32) -> BookResult<Self> {
        Ok(Self(InternalDate::from_ymd_opt(year, month, day).ok_or_else(|| BookError::new("Failed to construct date"))?))
    }

    pub fn new_internal(date: InternalDate) -> Self{
        Self(date)
    }

    pub fn internal_date(&self) -> InternalDate {
        return self.0;
    }

    pub fn year(&self) -> i32 {
        return self.0.year();
    }

    pub fn month(&self) -> u32 {
        return self.0.month();
    }

    pub fn day(&self) -> u32 {
        return self.0.day();
    }

    pub fn id(&self) -> i32 {
        self.0.year()*10000 + (self.0.month() as i32)*100 + (self.0.day() as i32)
    }

    pub fn from_str(s: &str) -> BookResult<Date> {
        let naive = chrono::NaiveDate::parse_from_str(s, DATE_STRING_FORMAT).map_err(|e| BookError::new(format!("Failed to parse date {} with {}", s, e)))?;
        Ok(Date(naive))
    }

    pub fn first_day_this_month(&self) -> BookResult<Self> {
        Ok(Self(chrono::NaiveDate::from_ymd_opt(self.0.year(), self.0.month(), 1).ok_or_else(|| BookError::new("No first day of last month"))?))
    }

    pub fn first_day_next_month(&self) -> BookResult<Self> {
        Ok(Self(
            chrono::NaiveDate::from_ymd_opt( self.0.year(), self.0.month(), 1).ok_or_else( | | BookError::new("No first day of month")) ?
            .checked_add_months(chrono::Months::new(1)).ok_or_else( || BookError::new("No first day of next month")) ?
        ))
    }

    pub fn last_day_this_month(&self) -> BookResult<Self> {
        Ok(Self(self.first_day_next_month()?.0.checked_sub_days(chrono::Days::new(1)).ok_or_else(|| BookError::new("No last day of this month"))?))
    }

    pub fn last_day_next_month(&self) -> BookResult<Self> {
        self.first_day_next_month()?.last_day_this_month()
    }

    pub fn previous_month(&self) -> BookResult<Period> {
        let first_day = self.0.checked_sub_months(chrono::Months::new(1)).ok_or_else(|| BookError::new("No first day of last month"))?;
        let last_day = self.0.checked_sub_days(chrono::Days::new(1)).ok_or_else(|| BookError::new("No last day of last month"))?;
        Ok(Period { begin: Date::new_internal(first_day), end: Date::new_internal(last_day) })
    }

    pub fn remaining_month(self) -> BookResult<Period> {
        Ok(Period{begin: self, end: self.last_day_this_month()?})
    }

    pub fn next_month_up_till(self, max_date: Date) -> BookResult<Option<Period>> {
        let begin = self.first_day_next_month()?;
        if begin>max_date {
            return Ok(None);
        }
        let end = std::cmp::min(self.last_day_next_month()?, max_date);
        Ok(Some(Period{begin, end}))
    }

    pub fn this_month(self) -> BookResult<Period> {
        Ok(Period { begin: self.first_day_this_month()?, end: self.last_day_this_month()?})
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
