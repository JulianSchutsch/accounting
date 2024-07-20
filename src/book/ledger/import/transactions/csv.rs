use std::collections::HashSet;

use crate::book::*;
use crate::book::formats::swedbank::konto::csv::Content;

fn add_account_periods(banks: &mut BankAccounts, content: &Content, path: &str) -> BookResult {
    let accounts = content.rows.iter().map(|row| row.account_nr).collect::<HashSet<i64>>();
    for account in accounts {
        let ref1 = BankAccountReference::SwedishAccountNumber(SwedishAccountNumber{number: account});
        let account = banks.get_mut_account_by_references(&BankAccountReferences::new_from_single(ref1.clone())).ok_or_else(|| BookError::new(format!("Account {} not defined!", ref1)))?;
        account.add_period(content.period, path.to_string());
    }
    Ok(())
}

fn add_account_entries(ledger: &mut Ledger, banks: &mut BankAccounts, content: Content) -> BookResult {
    for row in content.rows.into_iter() {
        let ref1 = BankAccountReference::SwedishAccountNumber(SwedishAccountNumber{number: row.account_nr});
        let event = Event::Transaction(Transaction{
            id: "".to_string(),
            date: row.transaction_date,
            account: ref1.clone(),
            amount: row.amount,
            references: BankTransactionReferences::new_from_single(row.reference.as_str())
        });
        ledger.events.insert(ledger.ledger_id.generate_transaction_id(row.transaction_date), event);
    }
    Ok(())
}

pub fn import(ledger: &mut Ledger, banks : &mut BankAccounts, path: &str, settings: &settings::banks::CSV) -> BookResult {
    if !settings.files.iter().any(|e| e.is_match(path)) {
        return Ok(());
    }

    let content = Content::import(path)?;
    add_account_periods(banks, &content, path)?;
    add_account_entries(ledger, banks, content)
}