use crate::book::*;

#[derive(Debug, Clone, Copy)]
pub enum BookAmount {
    Debit(Amount),
    Credit(Amount)
}

impl BookAmount {
    pub fn from_signed_amount(amount: Amount) -> Self {
        if amount<Amount(0.0) {
            Self::Credit(-amount)
        } else {
            Self::Debit(amount)
        }
    }

    pub fn invert(&self) -> Self {
        match self {
            BookAmount::Debit(a) => BookAmount::Credit(*a),
            BookAmount::Credit(a) => BookAmount::Debit(*a)
        }
    }

    pub fn plain_amount(&self) -> Amount {
        match self {
            BookAmount::Debit(a) => *a,
            BookAmount::Credit(a) => *a,
        }
    }
    pub fn signed_amount(&self) -> Amount {
        match self {
            BookAmount::Debit(a) => *a,
            BookAmount::Credit(a) => -*a,
        }
    }
}

impl std::fmt::Display for BookAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Debit(a) => write!(f, "Debit={}", a),
            Self::Credit(a) => write!(f, "Credit={}", a)
        }
    }
}
