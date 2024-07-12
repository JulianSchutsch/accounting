use crate::book::formats::skatteverket::konto::skv::Content;
use crate::book::*;

pub struct Importer {
    pub content: Content
}



impl LedgerImporter for Importer {
    fn import(&self, ledger: &mut Ledger) -> BookResult {
        let mut ledger_id = LedgerId::pseudo(self.content.period.begin);
        for row in self.content.rows.iter() {
            println!("Process tax row {} {}", row.description, row.date);
            if row.description.contains("Arbetsgivaravgift") {
                println!(" -> social");
                ledger.events.insert(ledger_id, Event::TaxPayment(TaxPayment{
                    id: row.description.clone(),
                    date: row.date,
                    amount: row.amount,
                    kind: TaxPaymentKind::SocialSecurityTax
                }));
                ledger_id.increase();
            }
            if row.description.contains("Avdragen skatt") {
                ledger.events.insert(ledger_id, Event::TaxPayment(TaxPayment{
                    id: row.description.clone(),
                    date: row.date,
                    amount: row.amount,
                    kind: TaxPaymentKind::EmployeeTax
                }));
                ledger_id.increase();
            }
        }
        Ok(())
    }
}
