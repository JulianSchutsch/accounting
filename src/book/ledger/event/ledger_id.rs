use crate::book::*;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum LedgerIdKind {
    Continuous(i32),
    Pseudo(i32)
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct LedgerId {
    pub fiscal_year_id: i32,
    pub id: LedgerIdKind
}

impl LedgerId {
    pub fn increase(&mut self) {
        self.id = match self.id {
            LedgerIdKind::Continuous(e) => LedgerIdKind::Continuous(e+1),
            LedgerIdKind::Pseudo(e) => LedgerIdKind::Pseudo(e+1)
        };
    }

    pub fn initial(date: Date) -> Self {
        Self{ fiscal_year_id: date.id(), id: LedgerIdKind::Continuous(0) }
    }

    pub fn pseudo(date: Date) -> Self {
        Self{ fiscal_year_id: date.id(), id: LedgerIdKind::Pseudo(0) }
    }
}

impl std::fmt::Display for LedgerIdKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LedgerIdKind::Continuous(e) => write!(f, "{}", e),
            LedgerIdKind::Pseudo(e) => write!(f, "P{}", e)
        }
    }
}

impl std::fmt::Display for LedgerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fiscal_start={} id={}", self.fiscal_year_id, self.id)
    }
}