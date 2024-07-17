use std::cmp::Ordering;
use crate::book::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LedgerIdKind {
    Technical,
    Pseudo,
    Continuous
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

    pub fn initial(date: Date) -> Self {
        Self{ fiscal_year_id: date.id(), id:0, kind: LedgerIdKind::Continuous }
    }

    pub fn pseudo(date: Date) -> Self {
        Self { fiscal_year_id: date.id(), id: 0, kind: LedgerIdKind::Pseudo }
    }
    pub fn technical(date: Date) -> Self {
        Self{ fiscal_year_id: date.id(), id:0, kind: LedgerIdKind::Technical }
    }
}

impl std::fmt::Display for LedgerIdKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LedgerIdKind::Continuous => write!(f, "normal"),
            LedgerIdKind::Pseudo => write!(f, "pseudo"),
            LedgerIdKind::Technical => write!(f, "technical")
        }
    }

}

impl std::fmt::Display for LedgerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fiscal_start={} id={} kind={}", self.fiscal_year_id, self.id, self.kind)
    }
}