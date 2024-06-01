use crate::book::bookresult::BookResult;
use super::ledger::Ledger;
use super::converter::Converter;
use super::bookaccounts::Accounts;

pub trait Generator {
    fn generate_accounts<'p:'r, 'r>(&self, converter: &dyn Converter, ledger: &'p Ledger) -> BookResult<Accounts<'r>>;
}
