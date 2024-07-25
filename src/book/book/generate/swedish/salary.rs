use crate::book::*;
use crate::book::book::generate::swedish::active_associables::ActiveAssociables;

use super::ids;
use super::params::Params;
use super::payment::*;

pub fn add(ledger_id: LedgerId, event: &Salary, p: &mut Params, associables: &mut ActiveAssociables) -> BookResult<()> {
    // "Utbetalning" TODO: Introduce type of salary, att the moment defaulted to service
    p.book.add_entry(ledger_id, event.date, &event.id, ids::SERVICE_SALARY, BookAmount::Debit(event.total));
    p.book.add_entry(ledger_id, event.date, &event.id, ids::EMPLOYEE_TAXES, BookAmount::Credit(event.employee_tax));

    let event_data = PaymentEventData {
        ledger_id,
        date: event.date,
        id: event.id.clone(),
        amount: -(event.total-event.employee_tax),
        currency: p.first.settings.book_currency,
        payments: event.payment.clone(),
        exchange_rate: None
    };
    process_payment(event_data, ids::SHORT_TERM_DEBT_SALARY, p, associables)?;

    // "Utbetalning", social security tax
    p.book.add_entry(ledger_id, event.date, &event.id, ids::EMPLOYER_SOCIAL_SECURITY_COSTS, BookAmount::Debit(event.employer_tax));
    p.book.add_entry(ledger_id, event.date, &event.id, ids::EMPLOYER_SOCIAL_SECURITY_TAX, BookAmount::Credit(event.employer_tax));

    // Declaration
    p.book.add_entry(ledger_id, event.declared, &event.id, ids::EMPLOYEE_TAXES, BookAmount::Debit(event.employee_tax));
    p.book.add_entry(ledger_id, event.declared, &event.id, ids::EMPLOYER_SOCIAL_SECURITY_TAX, BookAmount::Debit(event.employer_tax));
    p.book.add_entry(ledger_id, event.declared, &event.id, ids::SHORT_TERM_DEBT_TAXES, BookAmount::Credit(event.employer_tax+event.employee_tax));
    Ok(())
}