use std::cmp::Ordering;
use crate::book::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LedgerIdKind {
    Transaction,
    Verification,
    Time
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct LedgerId {
    pub fiscal_year_id: i32,
    pub id: i32,
    pub kind: LedgerIdKind
}

impl LedgerId {
    pub fn increase(&mut self) {
        self.id += 1
    }

    pub fn transactions(date: Date) -> Self {
        Self{ fiscal_year_id: date.id(), id:0, kind: LedgerIdKind::Transaction }
    }

    pub fn times(date: Date) -> Self {
        Self { fiscal_year_id: date.id(), id: 0, kind: LedgerIdKind::Time }
    }
    pub fn verifications(date: Date) -> Self {
        Self{ fiscal_year_id: date.id(), id:0, kind: LedgerIdKind::Verification }
    }
}

impl std::fmt::Display for LedgerIdKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LedgerIdKind::Verification => write!(f, "verification"),
            LedgerIdKind::Time => write!(f, "time"),
            LedgerIdKind::Transaction => write!(f, "transaction")
        }
    }

}

impl std::fmt::Display for LedgerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fiscal_start={} id={} kind={}", self.fiscal_year_id, self.id, self.kind)
    }
}