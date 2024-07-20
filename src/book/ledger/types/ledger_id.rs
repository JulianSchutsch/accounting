use crate::book::*;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct LedgerId {
    date: Date,
    fiscal_year_id: FiscalYearId,
    id: i32,
    kind: LedgerIdKind
}

impl LedgerId {
    pub fn new(date: Date, fiscal_year_id: FiscalYearId, id: i32, kind: LedgerIdKind) -> Self {
        Self{ date, fiscal_year_id, id, kind }
    }
}

impl std::fmt::Display for LedgerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}{}", self.fiscal_year_id.0, self.kind.shorthand(), self.id)
    }
}