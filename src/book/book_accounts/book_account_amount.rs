use crate::book::*;

#[derive(Debug, Clone, Copy)]
pub enum BookAccountAmount {
    Debit(Amount),
    Credit(Amount)
}

impl BookAccountAmount {
    pub fn from_signed_amount(amount: Amount) -> Self {
        if amount<Amount(0.0) {
            Self::Credit(-amount)
        } else {
            Self::Debit(amount)
        }
    }

    pub fn invert(&self) -> Self {
        match self {
            BookAccountAmount::Debit(a) => BookAccountAmount::Credit(*a),
            BookAccountAmount::Credit(a) => BookAccountAmount::Debit(*a)
        }
    }

    pub fn plain_amount(&self) -> Amount {
        match self {
            BookAccountAmount::Debit(a) => *a,
            BookAccountAmount::Credit(a) => *a,
        }
    }
    pub fn signed_amount(&self) -> Amount {
        match self {
            BookAccountAmount::Debit(a) => *a,
            BookAccountAmount::Credit(a) => -*a,
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
