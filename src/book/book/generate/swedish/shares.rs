use crate::book::*;
use crate::book::book::generate::swedish::active_associables::ActiveAssociables;

use super::ids;
use super::params::*;

use super::payment::*;

pub fn add<'p>(ledger_id: LedgerId, event: &Shares, p: &mut Params<'p>, associables: &mut ActiveAssociables<'p>) -> BookResult {
    p.book.add_entry(ledger_id, event.date, &event.id, ids::SHARES_CAPITAL, BookAmount::Credit(event.amount));

    let event_data = PaymentEventData{
        ledger_id,
        date: event.date,
        id: event.id.clone(),
        amount: event.amount,
        currency: p.first.exchange_rates.book_currency,
        payments: event.payment.clone(),
        exchange_rate: None
    };
    process_payment(event_data, ids::UNPAID_SHARES, p, associables)
}