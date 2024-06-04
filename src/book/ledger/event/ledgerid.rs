use crate::book::types::*;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct FiscalYearId(pub i32);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct LedgerId(pub i32, pub i32);

impl LedgerId {
    pub fn increase(&mut self) {
        self.1 += 1;
    }

    pub fn initial(date: Date) -> Self {
        Self(date.id(), 0)
    }
}