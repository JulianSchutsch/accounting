use num::traits::Inv;
use crate::book::*;

use super::params::Params;
use super::ids;
use super::active_associables::*;

struct AssociableInvoice {
    remaining: Amount,
    payments: Vec<Payment>
}

impl Associable<Transaction> for AssociableInvoice {
    fn associate(&mut self, data: &Transaction) -> BookResult<AssociableChange> {

        Ok((AssociableChange::Close))
    }

}

fn add_moms_sweden(p: &mut Params<Invoice>) -> BookResult {

    let amounts = p.event.amounts.convert_into_book_currency(p.event.date, &p.first.exchange_rates)?;

    for (&category, &amount) in amounts.iter() {
        let (incoming_moms_account, moms_factor) = ids::invoice_moms(category, amounts.reverse_charge)?;
        let invoice_account = ids::invoice_account(category)?;
        let moms = moms_factor*amount;
        p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, incoming_moms_account, BookAccountAmount::Debit(moms));
        p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, invoice_account, BookAccountAmount::Debit(amount));
    }
    Ok(())
}

fn add_moms(p: &mut Params<Invoice>) -> BookResult {
    if p.event.country.is_eu() {
        if p.event.country==Country::Sweden {
            add_moms_sweden(p)
        } else {
            panic!("Not implemented");
        }
    } else {
        panic!("Not implemented");
    }
}

fn add_immediate_payments(p: &mut Params<Invoice>) -> BookResult<(Vec<Payment>, Amount)> {
    let mut remaining_amount = p.event.amounts.total;
    let mut result: Vec<Payment> = Vec::new();
    // TODO: Near 0 is possible
    for payment in p.event.payment.iter() {
        match payment {
            Payment::Exact => {
                result.push(payment.clone())
            },
            Payment::LeaderCredit => {
                p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::DEBT_TO_PRIVATE, BookAccountAmount::Credit(remaining_amount));
                remaining_amount = Amount(0.0);
            }
        }
        if remaining_amount == Amount(0.0) {
            break;
        }
    }
    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::CLAIMS_FROM_CUSTOMERS, BookAccountAmount::Credit(remaining_amount));
    Ok((result, remaining_amount))
}

pub fn add(p: &mut Params<Invoice>) -> BookResult {
    add_moms(p)?;
    let (payments, remaining) = add_immediate_payments(p)?;
    if !payments.is_empty() {
        println!("Add associable {:?}", p.event);
        p.associables.transactions.register(Box::new(AssociableInvoice{
            remaining,
            payments
        }));
    }
    Ok(())
}