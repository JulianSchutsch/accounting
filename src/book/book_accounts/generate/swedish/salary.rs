use crate::book::*;

use super::params::Params;
use super::ids;

pub fn add(p: Params<Salary>) -> BookResult<()> {
    // "Utbetalning" TODO: Introduce type of salary, att the moment defaulted to service
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::SERVICE_SALARY, BookAccountAmount::Debit(p.event.total));
    // Hacking in as debt here. This is a short term debt
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::SHORT_TERM_DEBT_SALARY, BookAccountAmount::Credit(p.event.total-p.event.employee_tax));
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::EMPLOYEE_TAXES, BookAccountAmount::Credit(p.event.employee_tax));

    // "Utbetalning", social security tax
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::EMPLOYER_SOCIAL_SECURITY_COSTS, BookAccountAmount::Debit(p.event.employer_tax));
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::EMPLOYER_SOCIAL_SECURITY_TAX, BookAccountAmount::Credit(p.event.employer_tax));

    // Declaration
    p.second.book_accounts.add_entry(p.ledger_id, p.event.declared, &p.event_ref, ids::EMPLOYEE_TAXES, BookAccountAmount::Debit(p.event.employee_tax));
    p.second.book_accounts.add_entry(p.ledger_id, p.event.declared, &p.event_ref, ids::EMPLOYER_SOCIAL_SECURITY_TAX, BookAccountAmount::Debit(p.event.employer_tax));
    p.second.book_accounts.add_entry(p.ledger_id, p.event.declared, &p.event_ref, ids::SHORT_TERM_DEBT_TAXES, BookAccountAmount::Credit(p.event.employer_tax+p.event.employee_tax));
    Ok(())
}