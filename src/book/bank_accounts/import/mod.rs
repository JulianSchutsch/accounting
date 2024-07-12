mod csv;
mod skv;

use crate::book::*;

use super::bank_accounts::BankAccounts;

fn import_banks(banks: &mut BankAccounts, path: &str, fiscal_year_settings: &settings::FiscalYear) -> BookResult<Vec<Box<dyn LedgerImporter>>> {
    let mut result = Vec::<Box<dyn LedgerImporter>>::new();
    for banks_filter in fiscal_year_settings.banks.iter() {
        match banks_filter {
            settings::banks::Banks::CSV(e) => csv::import(banks, path, e).map_err(|e| e.extend(format!("Failed to import csv account from file {}", path)))?,
            settings::banks::Banks::SKV(e) => if let Some(importer) = skv::import(banks, path, e).map_err(|e| e.extend(format!("Failed to import skv account from file {}", path)))? {
                result.push(importer);
            }
        }
    }
    Ok(result)
}

fn import_fiscal_year(mut ledger: &mut BankAccounts, fiscal_year_settings: &settings::FiscalYear) -> BookResult<Vec<Box<dyn LedgerImporter>>> {
    let mut result = Vec::<Box<dyn LedgerImporter>>::new();
    utils::paths::directory_scan(std::path::Path::new(fiscal_year_settings.root_path.as_str()), &mut |path|{
        result.extend(import_banks(&mut ledger, path, fiscal_year_settings)?);
        Ok(())
    })?;
    Ok(result)
}

pub fn import_using_settings(settings: &settings::Settings) -> BookResult<(BankAccounts, Box<dyn LedgerImporter>)> {
    let mut banks = BankAccounts::new();
    for account in settings.accounts.iter() {
        banks.add_account(account.references.clone(), account.account_type, account.currency, account.initial_value)?;
    }
    let mut importer = Vec::<Box<dyn LedgerImporter>>::new();
    for fiscal_year in settings.fiscal_years.iter() {
        importer.extend(import_fiscal_year(&mut banks, fiscal_year)?);
    }
    let importer = Box::new(MultiLedgerImporter::new(importer));
    Ok((banks, importer))
}