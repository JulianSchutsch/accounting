use crate::book::*;

pub struct First {
    pub settings: settings::Settings,
    pub exchange_rates: ExchangeRates,
    pub bank_accounts: BankAccounts,
    pub ledger: Ledger,
}

impl First {
    pub fn from_root_file(path:&str) -> BookResult<Self> {
        let settings = settings::Settings::read_from_file(path)?;
        let exchange_rates = exchange_rate::import_using_settings(&settings)?;
        let mut bank_accounts = BankAccounts::new();
        let mut ledger = Ledger::new(settings.book_currency);
        ledger::import::import_using_settings(&mut ledger, &mut bank_accounts, &settings)?;
        Ok(Self { settings, exchange_rates, bank_accounts, ledger })
    }
}