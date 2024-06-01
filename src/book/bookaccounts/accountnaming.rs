use super::accountid::AccountId;

pub trait AccountNaming {
    fn name(&self, id: AccountId) -> Option<String>;
}