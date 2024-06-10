use super::date::Date;

#[derive(Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub struct DateRange {
    pub begin: Date,
    pub end: Date
}

impl DateRange {
    pub const FULL: DateRange = DateRange{ begin: Date::MIN, end: Date::MAX };

    pub fn new(begin: Date, end: Date) -> Self {
        Self { begin, end }
    }
    pub fn contains(&self, date: Date) -> bool {
        (date>=self.begin) && (date<=self.end)
    }
}

impl std::fmt::Display for DateRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.begin, self.end)
    }
}