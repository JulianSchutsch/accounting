mod csv;

use crate::book::bookresult::BookResult;
use crate::book::settings;
use crate::book::utils::paths::directory_scan;

use super::bankaccounts::BankAccounts;

fn import_banks(banks: &mut BankAccounts, path: &str, fiscal_year_settings: &settings::FiscalYear) -> BookResult<> {
    for banks_filter in fiscal_year_settings.banks.iter() {
        match banks_filter {
            settings::banks::Banks::CSV(e) => csv::import(banks, path, e)?
        }
    }
    Ok(())
}

fn import_fiscal_year(mut ledger: &mut BankAccounts, fiscal_year_settings: &settings::FiscalYear) -> BookResult<> {
    directory_scan(std::path::Path::new(fiscal_year_settings.root_path.as_str()), &mut |path|{
        import_banks(&mut ledger, path, fiscal_year_settings)
    })
}

pub fn import_using_settings(settings: &settings::Settings) -> BookResult<BankAccounts> {
    let mut banks = BankAccounts::new();
    for fiscal_year in settings.fiscal_years.iter() {
        import_fiscal_year(&mut banks, fiscal_year)?;
    }
    Ok(banks)
}