use crate::book::*;

use super::ids;
use super::params::*;

pub fn add(ledger_id: LedgerId, event: &TaxPayment, p: &mut Params) -> BookResult {
    match event.kind {
        TaxPaymentKind::EmployeeTax | TaxPaymentKind::SocialSecurityTax => {
            p.book.add_entry(ledger_id, event.date, &event.id, ids::SHORT_TERM_DEBT_TAXES, BookAmount::Debit(event.amount));
            p.book.add_entry(ledger_id, event.date, &event.id, ids::COMPANY_BANK_ACCOUNT, BookAmount::Credit(event.amount));
        },
        TaxPaymentKind::CompanyTax => {
            p.book.add_entry(ledger_id, event.date, &event.id, ids::PRELIMINARY_PAID_COMPANY_TAX, BookAmount::Debit(event.amount));
            p.book.add_entry(ledger_id, event.date, &event.id, ids::COMPANY_BANK_ACCOUNT, BookAmount::Credit(event.amount));
        },
        TaxPaymentKind::Moms => {
            if event.amount>=Amount(0.0) {
                p.book.add_entry(ledger_id, event.date, &event.id, ids::MOMS_DEBT, BookAmount::Debit(event.amount));
                p.book.add_entry(ledger_id, event.date, &event.id, ids::COMPANY_BANK_ACCOUNT, BookAmount::Credit(event.amount));
            } else {
                p.book.add_entry(ledger_id, event.date, &event.id, ids::MOMS_DEBT, BookAmount::Credit(-event.amount));
                p.book.add_entry(ledger_id, event.date, &event.id, ids::COMPANY_BANK_ACCOUNT, BookAmount::Debit(-event.amount));
            }
        }
    }
    Ok(())
}