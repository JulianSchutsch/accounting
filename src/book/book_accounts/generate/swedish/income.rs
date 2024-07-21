use crate::book::*;
use crate::book::book_accounts::generate::swedish::active_associables::ActiveAssociables;

use super::ids;
use super::params::*;

struct AssociableIncome {
    remaining: Amount,
    payments: Vec<Payment>
}

impl Associable<Transaction, BookAccounts> for AssociableIncome {
    fn associate(&mut self, ledger_id: LedgerId, data: &Transaction, book_accounts: &mut BookAccounts) -> BookResult<AssociableChange> {
        for payment in self.payments.iter() {
            match payment {
                Payment::Exact => {
                    // TODO: On close, is there no action required?
                }
                _ => return Err(BookError::new("Unsupported case for payment, implementation error"))
            }
        }
        Ok(AssociableChange::Close)
    }

}

fn add_moms_worldwide(ledger_id: LedgerId, event: &Income, first: &phases::First, book_accounts: &mut BookAccounts) -> BookResult {
    let amounts = event.amounts.convert_into_book_currency(event.date, &first.exchange_rates)?;

    let mut accumulated_moms = Amount(0.0);
    for (&category, &amount) in amounts.iter() {
        let (outgoing_moms_account, moms_factor) = ids::income_moms(category, amounts.reverse_charge)?;
        let income_account = ids::income_worldwide_account(category)?;
        let moms = moms_factor * amount;
        accumulated_moms+=moms;
        book_accounts.add_entry(ledger_id, event.date, &event.id, income_account, BookAccountAmount::Credit(amount));
        book_accounts.add_entry(ledger_id, event.date, &event.id, outgoing_moms_account, BookAccountAmount::Debit(moms));
    }
    if amounts.reverse_charge {
        book_accounts.add_entry(ledger_id, event.date, &event.id, ids::INCOMING_MOMS_PROCUREMENT_ABROAD, BookAccountAmount::Credit(accumulated_moms));
    }
    Ok(())
}

fn add_immediate(ledger_id: LedgerId, event: &Income, first: &phases::First, book_accounts: &mut BookAccounts) -> BookResult<(Vec<Payment>, Amount)> {
    let remaining = event.amounts.total;
    let result: Vec<Payment> = Vec::new();

    book_accounts.add_entry(ledger_id, event.date, &event.id, ids::CLAIMS_TO_CUSTOMERS, BookAccountAmount::Debit(remaining));
    Ok((result, remaining))
}

pub fn add(ledger_id: LedgerId, event: &Income, p: &mut Params) -> BookResult {
    assert_eq!(event.country.is_eu(), false);
    add_moms_worldwide(ledger_id, event, p.first, &mut p.book).map_err(|e| e.extend("Failed to add world wide income"))?;
    let (payments, remaining) = add_immediate(ledger_id, event, p.first, &mut p.book)?;
    if !payments.is_empty() {

    }
    Ok(())
}
