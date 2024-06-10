#[derive(Hash, Eq, Clone, Copy, PartialEq, PartialOrd, Ord, Debug)]
pub struct BookAccountId(pub i32);

impl BookAccountId {
    pub const MAX: BookAccountId = BookAccountId(i32::MAX);
    pub const MIN: BookAccountId = BookAccountId(i32::MIN);
}

impl std::fmt::Display for BookAccountId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}