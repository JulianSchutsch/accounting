#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LedgerIdKind {
    Transaction,
    Verification,
    Time
}

impl LedgerIdKind {
    pub fn shorthand(&self) -> char {
        match self {
            Self::Transaction => 't',
            Self::Verification => 'v',
            Self::Time => 'd'
        }
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
