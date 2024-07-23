use crate::book::*;

pub struct BookAccountSum {
    pub debit: Amount,
    pub credit: Amount,
}

impl BookAccountSum {
    const MAX_ERROR: Amount = Amount(0.02);
    pub fn zero() -> Self {
        Self { debit: Amount(0.0), credit: Amount(0.0) }
    }
    pub fn valid(&self) -> bool {
        return (self.debit.0-self.credit.0).abs()<=Self::MAX_ERROR.0;
    }
}

impl std::ops::AddAssign<BookAmount> for BookAccountSum {
    fn add_assign(&mut self, rhs: BookAmount) {
        match rhs {
            BookAmount::Debit(s) => self.debit += s,
            BookAmount::Credit(s) => self.credit += s
        }
    }
}