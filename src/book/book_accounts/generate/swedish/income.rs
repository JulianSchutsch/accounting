use crate::book::*;

use super::params::Params;
use super::ids;

struct AssociableIncome {
    remaining: Amount,
    payments: Vec<Payment>
}

impl Associable<Transaction> for AssociableIncome {
    fn associate(&mut self, data: &Transaction) -> BookResult<AssociableChange> {
        for payment in self.payments.iter() {
            match payment {
                Payment::Exact => {
                }
                _ => return Err(BookError::new("Unsupported case for payment, implementation error"))
            }
        }
        Ok((AssociableChange::Close))
    }

}

fn add_moms_worldwide(p: &mut Params<Income>) -> BookResult {
    let amounts = p.event.amounts.convert_into_book_currency(p.event.date, &p.first.exchange_rates)?;

    let mut accumulated_moms = Amount(0.0);
    for (&category, &amount) in amounts.iter() {
        let (outgoing_moms_account, moms_factor) = ids::income_moms(category, amounts.reverse_charge)?;
        let income_account = ids::income_worldwide_account(category)?;
        let moms = moms_factor * amount;
        accumulated_moms+=moms;
        p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, income_account, BookAccountAmount::Credit(amount));
        p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, outgoing_moms_account, BookAccountAmount::Debit(moms));
    }
    if amounts.reverse_charge {
        p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::INCOMING_MOMS_PROCUREMENT_ABROAD, BookAccountAmount::Credit(accumulated_moms));
    }
    Ok(())
}

fn add_immediate(p: &mut Params<Income>) -> BookResult<(Vec<Payment>, Amount)> {
    let remaining = p.event.amounts.total;
    let result: Vec<Payment> = Vec::new();

    p.second.book_accounts.add_entry(p.ledger_id, p.event.date, &p.event_ref, ids::CLAIMS_TO_CUSTOMERS, BookAccountAmount::Debit(remaining));
    Ok((result, remaining))
}

pub fn add(p: &mut Params<Income>) -> BookResult {
    assert_eq!(p.event.country.is_eu(), false);
    add_moms_worldwide(p).map_err(|e| e.extend("Failed to add world wide income"))?;
    let (payments, remaining) = add_immediate(p)?;
    if !payments.is_empty() {

    }
    Ok(())
}
