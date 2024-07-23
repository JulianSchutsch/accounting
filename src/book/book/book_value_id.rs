#[derive(Hash, Eq, Clone, Copy, PartialEq, PartialOrd, Ord, Debug)]
pub struct BookValueId(pub i32);

impl BookValueId {
    pub const MAX: BookValueId = BookValueId(i32::MAX);
    pub const MIN: BookValueId = BookValueId(i32::MIN);
}

impl std::fmt::Display for BookValueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}