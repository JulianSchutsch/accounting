use crate::book::*;

use super::ids;
use super::params::Params;

pub fn add(ledger_id: LedgerId, event: &Salary, p: &mut Params) -> BookResult<()> {
    // "Utbetalning" TODO: Introduce type of salary, att the moment defaulted to service
    p.book.add_entry(ledger_id, event.date, &event.id, ids::SERVICE_SALARY, BookAccountAmount::Debit(event.total));
    // Hacking in as debt here. This is a short term debt
    p.book.add_entry(ledger_id, event.date, &event.id, ids::SHORT_TERM_DEBT_SALARY, BookAccountAmount::Credit(event.total-event.employee_tax));
    p.book.add_entry(ledger_id, event.date, &event.id, ids::EMPLOYEE_TAXES, BookAccountAmount::Credit(event.employee_tax));

    // "Utbetalning", social security tax
    p.book.add_entry(ledger_id, event.date, &event.id, ids::EMPLOYER_SOCIAL_SECURITY_COSTS, BookAccountAmount::Debit(event.employer_tax));
    p.book.add_entry(ledger_id, event.date, &event.id, ids::EMPLOYER_SOCIAL_SECURITY_TAX, BookAccountAmount::Credit(event.employer_tax));

    // Declaration
    p.book.add_entry(ledger_id, event.declared, &event.id, ids::EMPLOYEE_TAXES, BookAccountAmount::Debit(event.employee_tax));
    p.book.add_entry(ledger_id, event.declared, &event.id, ids::EMPLOYER_SOCIAL_SECURITY_TAX, BookAccountAmount::Debit(event.employer_tax));
    p.book.add_entry(ledger_id, event.declared, &event.id, ids::SHORT_TERM_DEBT_TAXES, BookAccountAmount::Credit(event.employer_tax+event.employee_tax));
    Ok(())
}