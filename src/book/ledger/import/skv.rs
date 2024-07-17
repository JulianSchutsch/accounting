use crate::book::formats::skatteverket::konto::skv::{Row, Content};
use crate::book::*;

pub struct Importer {
    pub content: Content
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

impl LedgerImporter for Importer {
    fn import(&self, ledger: &mut Ledger) -> BookResult {
        let mut ledger_id = LedgerId::pseudo(self.content.period.begin);
        for row in self.content.rows.iter() {
            if !try_import_as_tax(&mut ledger_id, row, ledger) {
                if !try_import_as_interest(&mut ledger_id, row, ledger) {
                    try_import_as_fine(&mut ledger_id, row, ledger);
                }
            }
        }
        Ok(())
    }
}
