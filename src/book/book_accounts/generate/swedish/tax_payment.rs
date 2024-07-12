use crate::book::*;
use crate::book::book_accounts::generate::swedish::ids;
use crate::book::book_accounts::generate::swedish::params::Params;

pub fn add(p: Params<TaxPayment>) -> BookResult {
    match p.event.kind {
        TaxPaymentKind::EmployeeTax => {
            p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::EMPLOYEE_TAXES, BookAccountAmount::Debit(p.event.amount));
            p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::SHORT_TERM_DEBT_TAXES, BookAccountAmount::Credit(p.event.amount));
        }
        TaxPaymentKind::SocialSecurityTax => {
            p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::EMPLOYER_SOCIAL_SECURITY_TAX, BookAccountAmount::Debit(p.event.amount));
            p.second.book_accounts.add_entry(p.ledger_id, p.event.date, p.event_ref, ids::SHORT_TERM_DEBT_TAXES, BookAccountAmount::Credit(p.event.amount));
        }
    }
    Ok(())
}