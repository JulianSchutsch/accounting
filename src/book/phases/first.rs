use crate::book::*;

pub struct First {
    pub settings: settings::Settings,
    pub exchange_rates: ExchangeRates,
    pub bank_accounts: BankAccounts,
    pub ledger: Ledger
}

impl First {
    pub fn from_root_file(path:&str) -> BookResult<Self> {
        let settings = settings::Settings::read_from_file(path)?;
        let exchange_rates = exchange_rate::import_using_settings(&settings)?;
        let bank_accounts = bank_accounts::import::import_using_settings(&settings)?;
        let ledger = ledger::import_using_settings(&settings)?;
        Ok(Self { settings, exchange_rates, bank_accounts, ledger })
    }
}