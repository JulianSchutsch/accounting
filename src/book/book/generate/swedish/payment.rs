use crate::book::*;
use crate::book::book::generate::swedish::ids;

use super::active_associables::*;
use super::params::*;

pub struct PaymentEventData {
    pub ledger_id: LedgerId,
    pub date: Date,
    pub id: String,
    pub amount: Amount,
    pub currency: Currency,
    pub payments: Vec<Payment>,
    pub exchange_rate: Option<f64>
}

struct ExpectedTransaction {
    event_data: PaymentEventData,
    remaining: Amount,
    work_account: BookId
}

impl ExpectedTransaction {
    fn process_exact_payment(&mut self, ledger_id: LedgerId, data: &Transaction, p: &mut Params) -> BookResult {
        if data.currency != p.first.exchange_rates.book_currency {
            let converted_original_date = p.first.exchange_rates.convert_into_book_currency(self.event_data.date, data.currency, self.remaining, self.event_data.exchange_rate)?;
            let converted_today = p.first.exchange_rates.convert_into_book_currency(data.date, data.currency, self.remaining, None)?;
            let converted_difference = converted_original_date - converted_today;

            let book_original_date = BookAmount::from_signed_amount(-converted_original_date);
            let book_today = BookAmount::from_signed_amount(converted_today);
            let book_difference = BookAmount::from_signed_amount(converted_difference);
            p.book.add_entry(ledger_id, data.date, &self.event_data.id, self.work_account, book_original_date);
            p.book.add_entry(ledger_id, data.date, &self.event_data.id, ids::COMPANY_CURRENCY_ACCOUNT, book_today);
            p.book.add_entry(ledger_id, data.date, &self.event_data.id, ids::EXCHANGE_RATE_DIFFERENCES, book_difference);
        } else {
            let book_amount = BookAmount::from_signed_amount(self.remaining);
            p.book.add_entry(ledger_id, data.date, &self.event_data.id, self.work_account, book_amount.invert());
            p.book.add_entry(ledger_id, data.date, &self.event_data.id, ids::COMPANY_BANK_ACCOUNT, book_amount);
        }
        Ok(())
    }
}

impl Associable<Transaction, Params<'_>> for ExpectedTransaction {

    fn associate(&mut self, ledger_id: LedgerId, data: &Transaction, p: &mut Params) -> BookResult<AssociableChange> {
        for payment in self.event_data.payments.iter() {
            match payment {
                Payment::Exact => {
                    if data.currency == self.event_data.currency && data.amount == self.remaining {
                        self.process_exact_payment(ledger_id, data, p)?;
                        return Ok(AssociableChange::Close);
                    }
                },
                Payment::ExactExchanged(e) => {
                    if data.currency == p.first.exchange_rates.book_currency && almost_equal(data.amount, e.exchanged) {
                        self.process_exact_payment(ledger_id, data, p)?;
                        return Ok(AssociableChange::Close);
                    }
                }
                Payment::ExactMedCost(cost) => {
                    if data.currency == self.event_data.currency && data.amount == self.remaining-cost.cost {
                        let book_cost = BookAmount::from_signed_amount(cost.cost);
                        p.book.add_entry(ledger_id, data.date, &data.id, ids::BANK_COSTS, book_cost);
                        p.book.add_entry(ledger_id, data.date, &data.id, self.work_account, book_cost.invert());
                        self.remaining = self.remaining - cost.cost;
                        self.process_exact_payment(ledger_id, data, p)?;
                        return Ok(AssociableChange::Close);
                    }
                }
                _ => ()
            }
        }
        Ok(AssociableChange::NoMatch)
    }
}

fn calculate_immediate_payments(event_data: PaymentEventData, work_id: BookId, p: &mut Params) -> BookResult<ExpectedTransaction> {
    let mut remaining_amount = event_data.amount;
    // TODO: Near 0 is possible
    for payment in event_data.payments.iter() {
        match payment {
            Payment::LeaderCredit => {
                if remaining_amount>=Amount::zero() {
                    return Err(BookError::new("Unexpected to solve an invoice using LeaderCredit"));
                }
                p.book.add_entry(event_data.ledger_id, event_data.date, &event_data.id, ids::DEBT_TO_COMPANY_OWNERS, BookAmount::Credit(-remaining_amount));
                remaining_amount = Amount(0.0);
            },
            _ => ()
        }
        if remaining_amount == Amount(0.0) {
            break;
        }
    }
    let converted_remaining_amount = p.first.exchange_rates.convert_into_book_currency(event_data.date, event_data.currency, remaining_amount, event_data.exchange_rate)?;
    if !almost_equal(converted_remaining_amount, Amount::zero()) {
        p.book.add_entry(event_data.ledger_id, event_data.date, &event_data.id, work_id, BookAmount::from_signed_amount(converted_remaining_amount));
    }
    Ok(ExpectedTransaction{event_data, remaining: remaining_amount, work_account: work_id})
}

pub fn process_payment(event_data: PaymentEventData, work_account: BookId, p: &mut Params, associables: &mut ActiveAssociables) -> BookResult {
    let expected = calculate_immediate_payments(event_data, work_account, p)?;
    if expected.remaining!=Amount(0.0) {
        associables.transactions.register(Box::new(expected));
    }
    Ok(())
}