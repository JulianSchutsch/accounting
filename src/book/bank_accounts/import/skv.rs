use crate::book::*;
use crate::book::formats::skatteverket::konto::skv::Content;

fn add_account_entries(banks: &mut BankAccounts, content: &Content) -> BookResult {
    let account = banks.get_mut_account_by_references(&BankAccountReferences::new_from_single(content.s_ref.clone()))
        .ok_or_else(|| BookError::new(format!("Account {} not defined!", content.s_ref)))?;
    for row in content.rows.iter() {
        account.add_transaction(row.date, row.amount, BankTransactionReferences::new_from_single(row.description.as_str()))?;
    }
    Ok(())
}

pub fn import(banks : &mut BankAccounts, path: &str, settings: &settings::banks::SKV) -> BookResult<Option<Box<dyn LedgerImporter>>> {
    if !settings.files.iter().any(|e| e.is_match(path)) {
        return Ok(None);
    }
    let content = Content::import(path)?;
    add_account_entries(banks, &content)?;
    Ok(Some(Box::new(crate::book::ledger::import::skv::Importer{content})))
}