use crate::book::*;
use crate::book::formats::skatteverket::konto::skv::{Row, Content};

fn add_account_entries(banks: &mut BankAccounts, content: &Content) -> BookResult {
    let account = banks.get_mut_account_by_references(&BankAccountReferences::new_from_single(content.s_ref.clone()))
        .ok_or_else(|| BookError::new(format!("Account {} not defined!", content.s_ref)))?;
    for row in content.rows.iter() {
        account.add_transaction(row.date, row.amount, BankTransactionReferences::new_from_single(row.description.as_str()))?;
    }
    Ok(())
}

static IMPORTERKEYS_TAX:[(&str, TaxPaymentKind); 4]=[
    ("Arbetsgivaravgift", TaxPaymentKind::SocialSecurityTax),
    ("Avdragen skatt", TaxPaymentKind::EmployeeTax),
    ("Debiterad preliminärskatt", TaxPaymentKind::CompanyTax),
    ("Moms", TaxPaymentKind::Moms)
];

fn try_import_as_tax(ledger_id: &mut LedgerId, row: &Row, ledger: &mut Ledger) -> bool {
    for (key, kind) in IMPORTERKEYS_TAX.iter() {
        if row.description.contains(key) {
            ledger.events.insert(*ledger_id, Event::TaxPayment(TaxPayment{
                id: row.description.clone(),
                date: row.date,
                amount: -row.amount,
                kind: *kind
            }));
            ledger_id.increase();
            return true;
        }
    }
    false
}

fn try_import_as_interest(ledger_id: &mut LedgerId, row: &Row, ledger: &mut Ledger) -> bool {
    if row.description.contains("Intäktsränta") {
        ledger.events.insert(*ledger_id, Event::Interest(Interest{
            id: row.description.clone(),
            date: row.date,
            amount: row.amount,
            taxable: false
        }));
        ledger_id.increase();
        return true;
    }
    false
}

fn try_import_as_fine(ledger_id: &mut LedgerId, row: &Row, ledger: &mut Ledger) -> bool {
    if row.description.contains("Kostnadsränta") {
        ledger.events.insert(*ledger_id, Event::Fine(Fine{
            id: row.description.clone(),
            date: row.date,
            amount: -row.amount,
        }));
        ledger_id.increase();
        return true;
    }
    false
}

fn import_transactions(content: Content, ledger: &mut Ledger) -> BookResult {
    let mut ledger_id = LedgerId::transactions(content.period.begin);
    for row in content.rows.iter() {
        if !try_import_as_tax(&mut ledger_id, row, ledger) {
            if !try_import_as_interest(&mut ledger_id, row, ledger) {
                try_import_as_fine(&mut ledger_id, row, ledger);
            }
        }
    }
    Ok(())
}

pub fn import(ledger: &mut Ledger, banks : &mut BankAccounts, path: &str, settings: &settings::banks::SKV) -> BookResult {
    if !settings.files.iter().any(|e| e.is_match(path)) {
        return Ok(());
    }
    let content = Content::import(path)?;
    add_account_entries(banks, &content)?;
    import_transactions(content, ledger)?;
    Ok(())
}