use crate::book::*;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct LedgerId {
    fiscal_year_id: FiscalYearId,
    id: i32,
}

impl LedgerId {
    pub fn new(fiscal_year_id: FiscalYearId, id: i32) -> Self {
        Self{ fiscal_year_id, id }
    }
}

impl std::fmt::Display for LedgerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.fiscal_year_id.0, self.id)
    }
}