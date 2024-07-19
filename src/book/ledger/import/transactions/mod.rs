mod csv;
mod skv;

use crate::book::*;

fn import_banks(ledger: &mut Ledger, banks: &mut BankAccounts, path: &str, fiscal_year_settings: &settings::FiscalYear) -> BookResult {
    for banks_filter in fiscal_year_settings.banks.iter() {
        match banks_filter {
            settings::banks::Banks::CSV(e) => csv::import(ledger, banks, path, e).map_err(|e| e.extend(format!("Failed to import csv account from file {}", path)))?,
            settings::banks::Banks::SKV(e) => skv::import(ledger, banks, path, e).map_err(|e| e.extend(format!("Failed to import skv account from file {}", path)))?
        }
    }
    Ok(())
}

pub fn import_fiscal_year(ledger: &mut Ledger, banks: &mut BankAccounts, fiscal_year_settings: &settings::FiscalYear) -> BookResult {
    utils::paths::directory_scan(std::path::Path::new(fiscal_year_settings.root_path.as_str()), &mut |path|{
        import_banks(ledger, banks, path, fiscal_year_settings)?;
        Ok(())
    })?;
    Ok(())
}

pub fn add_accounts(banks: &mut BankAccounts, settings: &settings::Settings) -> BookResult {
    for account in settings.accounts.iter() {
        banks.add_account(account.references.clone(), account.account_type, account.currency, account.initial_value)?;
    }
    Ok(())
}