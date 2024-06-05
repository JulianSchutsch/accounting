use super::date::Date;

#[derive(Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub struct DateRange {
    pub begin: Date,
    pub end: Date
}

impl DateRange {
    pub fn contains(&self, date: Date) -> bool {
        (date>=self.begin) && (date<=self.end)
    }
}

impl std::fmt::Display for DateRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.begin, self.end)
    }
}