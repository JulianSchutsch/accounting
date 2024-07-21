use crate::book::*;

use super::ids;
use super::active_associables::*;
use super::params::*;

struct AssociableInvoice {
    remaining: Amount,
    payments: Vec<Payment>
}

impl Associable<Transaction, Params<'_>> for AssociableInvoice {
    fn associate(&mut self, ledger_id: LedgerId, data: &Transaction, p: &mut Params) -> BookResult<AssociableChange> {
        // TODO: Actually absorb payments!
        Ok((AssociableChange::Close))
    }

}

fn add_moms_sweden(ledger_id: LedgerId, event: &Invoice, first: &phases::First, book_accounts: &mut BookAccounts) -> BookResult {

    let amounts = event.amounts.convert_into_book_currency(event.date, &first.exchange_rates)?;

    for (&category, &amount) in amounts.iter() {
        let (incoming_moms_account, moms_factor) = ids::invoice_moms(category, amounts.reverse_charge)?;
        let invoice_account = ids::invoice_account(category)?;
        let moms = moms_factor*amount;
        book_accounts.add_entry(ledger_id, event.date, &event.id, incoming_moms_account, BookAccountAmount::Debit(moms));
        book_accounts.add_entry(ledger_id, event.date, &event.id, invoice_account, BookAccountAmount::Debit(amount));
    }
    Ok(())
}

fn add_moms(ledger_id: LedgerId, event: &Invoice, first: &phases::First, book_accounts: &mut BookAccounts) -> BookResult {
    if event.country.is_eu() {
        if event.country==Country::Sweden {
            add_moms_sweden(ledger_id, event, first, book_accounts)
        } else {
            panic!("Not implemented");
        }
    } else {
        panic!("Not implemented");
    }
}

fn add_immediate_payments(ledger_id: LedgerId, event: &Invoice, first: &phases::First, book_accounts: &mut BookAccounts) -> BookResult<(Vec<Payment>, Amount)> {
    let mut remaining_amount = event.amounts.total;
    let mut result: Vec<Payment> = Vec::new();
    // TODO: Near 0 is possible
    for payment in event.payment.iter() {
        match payment {
            Payment::Exact => {
                result.push(payment.clone())
            },
            Payment::LeaderCredit => {
                book_accounts.add_entry(ledger_id, event.date, &event.id, ids::DEBT_TO_COMPANY_OWNERS, BookAccountAmount::Credit(remaining_amount));
                remaining_amount = Amount(0.0);
            }
        }
        if remaining_amount == Amount(0.0) {
            break;
        }
    }
    book_accounts.add_entry(ledger_id, event.date, &event.id, ids::CLAIMS_FROM_CUSTOMERS, BookAccountAmount::Credit(remaining_amount));
    Ok((result, remaining_amount))
}

pub fn add(ledger_id: LedgerId, event: &Invoice, p: &mut Params) -> BookResult {
    add_moms(ledger_id, event, p.first, &mut p.book)?;
    let (payments, remaining) = add_immediate_payments(ledger_id, event, p.first, &mut p.book)?;
    if !payments.is_empty() {
        println!("Add associable {:?}", event);
/*        p.associables.transactions.register(Box::new(AssociableInvoice{
            remaining,
            payments
        }));*/
    }
    Ok(())
}