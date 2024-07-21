use crate::book::*;
use crate::book::formats::skatteverket::konto::skv::{Row, Content};

static IMPORTERKEYS_TAX:[(&str, TaxPaymentKind); 4]=[
    ("Arbetsgivaravgift", TaxPaymentKind::SocialSecurityTax),
    ("Avdragen skatt", TaxPaymentKind::EmployeeTax),
    ("Debiterad preliminärskatt", TaxPaymentKind::CompanyTax),
    ("Moms", TaxPaymentKind::Moms)
];

fn try_import_as_tax(row: &Row, ledger: &mut Ledger) -> bool {
    for (key, kind) in IMPORTERKEYS_TAX.iter() {
        if row.description.contains(key) {
            ledger.events.insert(ledger.ledger_id.generate_transaction_id(row.date), Event::TaxPayment(TaxPayment{
                id: row.description.clone(),
                date: row.date,
                amount: -row.amount,
                kind: *kind
            }));
            return true;
        }
    }
    false
}

fn try_import_as_interest(row: &Row, ledger: &mut Ledger) -> bool {
    if row.description.contains("Intäktsränta") {
        ledger.events.insert(ledger.ledger_id.generate_transaction_id(row.date), Event::Interest(Interest{
            id: row.description.clone(),
            date: row.date,
            currency: ledger.book_currency,
            amount: row.amount,
            taxable: false
        }));
        return true;
    }
    false
}

fn try_import_as_fine(row: &Row, ledger: &mut Ledger) -> bool {
    if row.description.contains("Kostnadsränta") {
        ledger.events.insert(ledger.ledger_id.generate_transaction_id(row.date), Event::Fine(Fine{
            id: row.description.clone(),
            date: row.date,
            amount: -row.amount,
        }));
        return true;
    }
    false
}

fn import_transactions(content: &Content, ledger: &mut Ledger) -> BookResult {
    for row in content.rows.iter() {
        if !try_import_as_tax(row, ledger) {
            if !try_import_as_interest(row, ledger) {
                if !try_import_as_fine(row, ledger) {
                    if row.description.contains("Inbetalning bokförd") {
                         ledger.events.insert(ledger.ledger_id.generate_transaction_id(row.date), Event::Transaction(Transaction {
                            id: "".to_string(),
                            date: row.date,
                            amount: row.amount,
                            references: BankTransactionReferences::new_from_single("Inbetalning bokförd"),
                            account: content.s_ref.clone()
                        }));
                    } else {
                        return Err(BookError::new("Unknown entry in skatteverket account"));
                    }
                }
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
    import_transactions(&content, ledger)?;
    Ok(())
}