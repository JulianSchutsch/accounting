#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventKind {
    Transaction,
    Verification,
    Check
}

impl std::fmt::Display for EventKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventKind::Verification => write!(f, "verification"),
            EventKind::Check => write!(f, "check"),
            EventKind::Transaction => write!(f, "transaction")
        }
    }
}
