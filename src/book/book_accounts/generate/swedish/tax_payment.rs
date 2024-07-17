use crate::book::*;

use super::ids;
use super::params::Params;

pub fn add(p: Params<TaxPayment>) -> BookResult {
    match p.event.kind {
        TaxPaymentKind::EmployeeTax | TaxPaymentKind::SocialSecurityTax => {
            p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::SHORT_TERM_DEBT_TAXES, BookAccountAmount::Debit(p.event.amount));
            p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::COMPANY_BANK_ACCOUNT, BookAccountAmount::Credit(p.event.amount));
        },
        TaxPaymentKind::CompanyTax => {
            p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::PRELIMINARY_PAID_COMPANY_TAX, BookAccountAmount::Debit(p.event.amount));
            p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::COMPANY_BANK_ACCOUNT, BookAccountAmount::Credit(p.event.amount));
        },
        TaxPaymentKind::Moms => {
            if p.event.amount>=Amount(0.0) {
                p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::MOMS_DEBT, BookAccountAmount::Debit(p.event.amount));
                p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::COMPANY_BANK_ACCOUNT, BookAccountAmount::Credit(p.event.amount));
            } else {
                p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::MOMS_DEBT, BookAccountAmount::Credit(-p.event.amount));
                p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::COMPANY_BANK_ACCOUNT, BookAccountAmount::Debit(-p.event.amount));
            }
        }
    }
    Ok(())
}