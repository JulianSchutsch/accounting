use crate::book::*;

pub mod verifications;
pub mod transactions;
pub mod times;

pub fn import_using_settings(ledger: &mut Ledger, banks: &mut BankAccounts, settings: &settings::Settings) -> BookResult {
    transactions::add_accounts(banks, settings)?;
    for fiscal_year in settings.fiscal_years.iter() {
        ledger.select_fiscal_year(fiscal_year.fiscal_year);
        verifications::import_fiscal_year(ledger, fiscal_year).map_err(|e| e.extend(format!("Failed to import fiscal year {}", fiscal_year.fiscal_year)))?;
        transactions::import_fiscal_year(ledger, banks, fiscal_year).map_err(|e| e.extend(format!("Failed to import fiscal year {}", fiscal_year.fiscal_year)))?;
        times::generate_fiscal_year(ledger, settings, fiscal_year).map_err(|e| e.extend(format!("Failed to generate fiscal year {}", fiscal_year.fiscal_year)))?;
    }
    Ok(())
}