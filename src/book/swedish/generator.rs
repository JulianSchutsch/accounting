pub struct Generator;

use crate::book::types::*;
use crate::book::accounts::Accounts;
use crate::book::ledger::Ledger;
use crate::book::event;
use super::konto;

impl Generator {
    fn add_income_to_accounts<'l>(accounts: &mut Accounts<'l>, e: &event::Income, f: &'l event::Event) {
        match e.method {
            event::income::Method::ReverseCharge => {
                accounts.add_entry(&e.date, &f, konto::kundfordringar, BookAmount(e.amount.0)); // TODO: Change currency!
//                accounts.add_entry(e.date, &f, );
            },
            _ => panic!("Not supported path"),
        }
    }
}

impl crate::book::generator::Generator for Generator {
    fn generate_accounts<'l>(&self, ledger: &'l Ledger) -> Result<Accounts<'l>, String> {
        let mut accounts = Accounts::new();
        for (date, entry) in ledger.iter() {
            match entry {
                event::Event::Income(e) => Self::add_income_to_accounts(&mut accounts, e, entry),
            }
        }
        Ok(accounts)
    }
}
