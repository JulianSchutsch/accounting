use crate::book::*;

#[derive(Debug, Clone, Copy)]
pub enum BookAccountAmount {
    Debit(Amount),
    Credit(Amount)
}

impl BookAccountAmount {
    pub fn plain_amount(&self) -> Amount {
        match self {
            BookAccountAmount::Credit(a) => *a,
            BookAccountAmount::Debit(a) => *a,
        }
    }
}

impl std::fmt::Display for BookAccountAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Debit(a) => write!(f, "Debit={}", a),
            Self::Credit(a) => write!(f, "Credit={}", a)
        }
    }
}
