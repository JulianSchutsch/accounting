use book::*;

use super::settings;
use super::bank_accounts;

pub fn ledger_id() -> LedgerId {
    LedgerId::initial(settings::fiscal_year().begin)
}

pub fn import() -> Import {
    let settings = settings::settings();
    let exchange_rates = ExchangeRates::new(Currency::SEK);
    let mut bank_accounts = BankAccounts::new();
    bank_accounts::bank_add(&mut bank_accounts).unwrap();
    let mut ledger = Ledger::new();
    Import { settings, exchange_rates, bank_accounts, ledger }
}