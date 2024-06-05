use crate::book::settings;
use crate::book::exchange_rate;
use crate::book::bank_accounts;
use crate::book::ledger;
use crate::book::book_result::BookResult;

pub struct Import {
    pub settings: settings::Settings,
    pub exchange_rates: exchange_rate::ExchangeRates,
    pub bank_accounts: bank_accounts::BankAccounts,
    pub ledger: ledger::Ledger
}

impl Import {
    pub fn from_root_file(path:&str) -> BookResult<Self> {
        let settings = settings::Settings::read_from_file(path)?;
        let exchange_rates = exchange_rate::import_using_settings(&settings)?;
        let bank_accounts = bank_accounts::import_using_settings(&settings)?;
        let ledger = ledger::import_using_settings(&settings)?;
        Ok(Self { settings, exchange_rates, bank_accounts, ledger })
    }
}