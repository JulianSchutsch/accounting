use crate::book::*;
use crate::book::book_accounts::generate::swedish::ids;

use super::active_associables::*;
use super::params::*;

pub struct PaymentEventData {
    pub ledger_id: LedgerId,
    pub date: Date,
    pub id: String,
    pub amount: Amount,
    pub currency: Currency,
    pub payments: Vec<Payment>
}

struct ExpectedTransaction {
    event_data: PaymentEventData,
    remaining: Amount,
    work_account: BookAccountId
}

impl Associable<Transaction, Params<'_>> for ExpectedTransaction {
    fn associate(&mut self, ledger_id: LedgerId, data: &Transaction, p: &mut Params) -> BookResult<AssociableChange> {
        println!("Assoc ? {}", data.amount);
        for payment in self.event_data.payments.iter() {
            match payment {
                Payment::Exact => {
                    if data.amount==self.event_data.amount {
                        let book_amount = BookAccountAmount::from_signed_amount(self.event_data.amount);
                        p.book.add_entry(ledger_id, self.event_data.date, &self.event_data.id, self.work_account, book_amount.invert());
                        p.book.add_entry(ledger_id, self.event_data.date, &self.event_data.id, ids::COMPANY_BANK_ACCOUNT, book_amount);
                    }
                },
                _ => ()
            }
        }
        // TODO: Actually absorb payments!
        Ok((AssociableChange::Close))
    }
}

fn calculate_immediate_payments(event_data: PaymentEventData, work_id: BookAccountId, book_accounts: &mut BookAccounts) -> BookResult<ExpectedTransaction> {
    let mut remaining_amount = event_data.amount;
    // TODO: Near 0 is possible
    for payment in event_data.payments.iter() {
        match payment {
            Payment::LeaderCredit => {
                book_accounts.add_entry(event_data.ledger_id, event_data.date, &event_data.id, ids::DEBT_TO_COMPANY_OWNERS, BookAccountAmount::Credit(remaining_amount));
                remaining_amount = Amount(0.0);
            },
            _ => ()
        }
        if remaining_amount == Amount(0.0) {
            break;
        }
    }
    book_accounts.add_entry(event_data.ledger_id, event_data.date, &event_data.id, work_id, BookAccountAmount::from_signed_amount(remaining_amount));
    Ok(ExpectedTransaction{event_data, remaining: remaining_amount, work_account: work_id})
}

pub fn process_payment(event_data: PaymentEventData, work_account: BookAccountId, book_accounts: &mut BookAccounts, associables: &mut ActiveAssociables) -> BookResult {
    let expected = calculate_immediate_payments(event_data, work_account, book_accounts)?;
    if expected.remaining!=Amount(0.0) {
        associables.transactions.register(Box::new(expected));
    }
    Ok(())
}