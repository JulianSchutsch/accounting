use crate::book::*;

use super::ids;
use super::params::Params;

pub fn add(ledger_id: LedgerId, event: &Salary, p: &mut Params) -> BookResult<()> {
    // "Utbetalning" TODO: Introduce type of salary, att the moment defaulted to service
    p.book.add_entry(ledger_id, event.date, &event.id, ids::SERVICE_SALARY, BookAmount::Debit(event.total));
    // Hacking in as debt here. This is a short term debt
    p.book.add_entry(ledger_id, event.date, &event.id, ids::SHORT_TERM_DEBT_SALARY, BookAmount::Credit(event.total-event.employee_tax));
    p.book.add_entry(ledger_id, event.date, &event.id, ids::EMPLOYEE_TAXES, BookAmount::Credit(event.employee_tax));

    // "Utbetalning", social security tax
    p.book.add_entry(ledger_id, event.date, &event.id, ids::EMPLOYER_SOCIAL_SECURITY_COSTS, BookAmount::Debit(event.employer_tax));
    p.book.add_entry(ledger_id, event.date, &event.id, ids::EMPLOYER_SOCIAL_SECURITY_TAX, BookAmount::Credit(event.employer_tax));

    // Declaration
    p.book.add_entry(ledger_id, event.declared, &event.id, ids::EMPLOYEE_TAXES, BookAmount::Debit(event.employee_tax));
    p.book.add_entry(ledger_id, event.declared, &event.id, ids::EMPLOYER_SOCIAL_SECURITY_TAX, BookAmount::Debit(event.employer_tax));
    p.book.add_entry(ledger_id, event.declared, &event.id, ids::SHORT_TERM_DEBT_TAXES, BookAmount::Credit(event.employer_tax+event.employee_tax));
    Ok(())
}