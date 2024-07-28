use chrono::Datelike;
use crate::book::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Deserialize)]
pub struct Period {
    pub begin : Date,
    pub end: Date
}

pub struct PeriodMonthIter {
    current: Option<Period>,
    max_date: Date
}

impl Iterator for PeriodMonthIter {
    type Item = Period;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;
        if result.is_some() {
            self.current = self.current.unwrap().end.next_month_up_till(self.max_date).unwrap();
        }
        result
    }
}

impl Period {
    pub const FULL: Period = Period{ begin: Date::MIN, end: Date::MAX };

    pub fn extend_to_min(&self) -> Period {
        Period{ begin: Date::MIN, end: self.end }
    }

    pub fn iterate_months(&self) -> PeriodMonthIter {
        PeriodMonthIter{ current: Some(self.begin.remaining_month().unwrap()), max_date: self.end}
    }

    pub fn new_invalid() -> Self {
        Self{begin: Date::MAX, end: Date::MIN}
    }

    pub fn new() -> Self {
        Self{begin: Date::MIN, end: Date::MIN}
    }

    pub fn from_dates(date1: Date, date2: Date) -> Self {
        Self{begin: date1, end: date2}
    }

    pub fn contains(&self, date: Date) -> bool {
        (date>=self.begin) && (date<=self.end)
    }

    pub fn last_within_period<Value, T: std::iter::Iterator>(&self, it: T, f: fn(&T::Item)->(Date, Value)) -> Option<Value> {
        let mut result : Option<Value>=None;
        let mut date = Date::MIN;
        for e in it {
            let (d, v) = f(&e);
            if (date<=d) && self.contains(d) {
                result = Some(v);
                date = d;
            }
        }
        return result;
    }

    pub fn first_within_period<Value, T: std::iter::Iterator>(&self, it: T, f: fn(&T::Item)->(Date, Value)) -> Option<Value> {
        let mut result : Option<Value>=None;
        let mut date = Date::MAX;
        for e in it {
            let (d, v) = f(&e);
            if (date>=d) && self.contains(d) {
                result = Some(v);
                date = d;
            }
        }
        return result;
    }

    pub fn month_till_end_max(date: &Date, max_date: &Date) -> BookResult<Self> {
        let first_day = date.internal_date();
        let first_day_next_month = chrono::NaiveDate::from_ymd_opt(first_day.year(), first_day.month(), 1).ok_or_else(|| BookError::new("No first day of next month"))?
            .checked_add_months(chrono::Months::new(1)).ok_or_else(|| BookError::new("No first day of next month"))?;
        let last_day = first_day_next_month.checked_sub_days(chrono::Days::new(1)).ok_or_else(|| BookError::new("No last day of this month"))?;
        if max_date.internal_date()<last_day {
            Ok(Self { begin: Date::new_internal(first_day), end: *max_date })
        } else {
            Ok(Self { begin: Date::new_internal(first_day), end: Date::new_internal(last_day) })
        }
    }

    pub fn first_month_in_period(&self) -> BookResult<Period> {
        Self::month_till_end_max(&self.begin, &self.end)
    }

    pub fn next_month_up_till(&mut self, date: &Date) -> BookResult<bool> {
        if *date<=self.end {
            return Ok(false);
        }
        let first_day_next_month = self.begin.internal_date().checked_add_months(chrono::Months::new(1)).ok_or_else(|| BookError::new("No first day of next month"))?;
        let last_day_next_month = first_day_next_month.checked_add_months(chrono::Months::new(1)).ok_or_else(|| BookError::new("No first day of month efter"))?
            .checked_sub_days(chrono::Days::new(1)).ok_or_else(|| BookError::new("No last day next month"))?;
        self.begin = Date::new_internal(first_day_next_month);
        self.end = Date::new_internal(last_day_next_month);
        Ok(true)
    }
}

impl std::fmt::Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.begin, self.end)
    }
}