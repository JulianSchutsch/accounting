#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BankAccountId(pub i32);

impl BankAccountId {
    pub fn increase(&mut self) -> Self {
        let result = Self(self.0);
        self.0+=1;
        result
    }
}
