use super::ledger::Ledger;
use super::accounts::Accounts;

pub trait Generator {
    fn generate_accounts<'l>(&self, ledger: &'l Ledger) -> Result<Accounts<'l>, String>;
}
