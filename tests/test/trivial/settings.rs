use book::*;

pub fn fiscal_year() -> DateRange {
    DateRange::new(Date::from_str("2000-01-01").unwrap(), Date::from_str("2000-12-31").unwrap())
}

pub fn settings() -> settings::Settings {
    let fiscal_year = settings::FiscalYear{
        root_path: String::new(),
        fiscal_year: fiscal_year(),
        banks: Vec::new(),
        events: Vec::new(),
        annual_accounts_method: settings::AnnualAccountsMethod::SwedishK2,
    };
    let account = settings::Account {
        account_type: BankAccountType::Account,
        initial_value: Amount(0.0),
        currency: Currency::SEK,
        references: BankAccountReferences::new_from_single(BankAccountReference::SwedishAccountNumber(SwedishAccountNumber{number: 1}))
    };
    settings::Settings {
        root_directory: String::new(),
        book_currency: Currency::SEK,
        book_generator: "swedish".to_string(),
        exchange_rates: Vec::new(),
        fiscal_years: vec![fiscal_year],
        accounts: vec![account]
    }
}