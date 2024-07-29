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
            let book_amount = BookAmount::from_signed_amount(event.amount);
            p.book.add_entry(ledger_id, event.date, &event.id, ids::MOMS_DEBT, book_amount);
            p.book.add_entry(ledger_id, event.date, &event.id, ids::COMPANY_BANK_ACCOUNT, book_amount.invert());
        }
    }
    Ok(())
}