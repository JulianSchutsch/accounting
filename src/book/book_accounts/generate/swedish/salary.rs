use crate::book::*;

use super::params::Params;
use super::ids;

pub fn add(p: Params<Salary>) -> BookResult<()> {
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::SERVICE_SALARY, BookAccountAmount::Debit(p.event.total));
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::SHORT_TERM_DEBT_SALARY, BookAccountAmount::Credit(p.event.total-p.event.employee_tax));
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::EMPLOYEE_PRELIMINARY_TAXES, BookAccountAmount::Credit(p.event.employee_tax));
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::EMPLOYER_COSTS, BookAccountAmount::Credit(p.event.employer_tax));
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::EMPLOYER_SOCIAL_FEES, BookAccountAmount::Debit(p.event.employer_tax));
    Ok(())
}