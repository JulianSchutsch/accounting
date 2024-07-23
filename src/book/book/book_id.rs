#[derive(Hash, Eq, Clone, Copy, PartialEq, PartialOrd, Ord, Debug)]
pub struct BookId(pub i32);

impl BookId {
    pub const MAX: BookId = BookId(i32::MAX);
    pub const MIN: BookId = BookId(i32::MIN);
}

impl std::fmt::Display for BookId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}