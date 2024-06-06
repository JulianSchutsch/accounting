use crate::book::*;

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

impl std::fmt::Display for LedgerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fiscal_start={} id={}", self.0, self.1)
    }
}