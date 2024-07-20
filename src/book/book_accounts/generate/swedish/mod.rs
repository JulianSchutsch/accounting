mod income;
mod invoice;
mod params;
mod ids;
mod naming;
mod salary;
mod tax_payment;
mod interest;
mod fine;
mod verify_moms;
mod shares;
mod transaction;
mod active_associables;

use crate::book::*;

use params::*;
use active_associables::*;

fn add_entry<'p, 'e:'p>(p: IncompleteParams<'_, 'p, 'e, '_>, entry: &'e Event) -> BookResult {
    match entry {
        Event::Income(e) => income::add(&mut p.complete_with(e, entry)).map_err(|e| e.extend("Failed to add income")),
        Event::Invoice(e) => invoice::add(&mut p.complete_with(e, entry)).map_err(|e| e.extend("Failed to add invoice")),
        Event::Salary(e) => salary::add(p.complete_with(e, entry)).map_err(|e| e.extend("Failed to add salary")),
        Event::TaxPayment(e) => tax_payment::add(p.complete_with(e, entry)).map_err(|e| e.extend("Failed to add tax payment")),
        Event::Interest(e) => interest::add(p.complete_with(e, entry)).map_err(|e| e.extend("Failed to add interest")),
        Event::Fine(e) => fine::add(p.complete_with(e, entry)).map_err(|e| e.extend("Failed to add fine")),
        Event::VerifyMoms(e) => verify_moms::add(p.complete_with(e, entry)).map_err(|e| e.extend("Failed to add verify moms")),
        Event::Shares(e) => shares::add(p.complete_with(e, entry)).map_err(|e| e.extend("Failed to add shares")),
        Event::Transaction(e) => transaction::add(&mut p.complete_with(e, entry)).map_err(|e| e.extend("Failed to add transaction"))
    }
}

pub fn generate(first: &phases::First) -> BookResult<phases::Second> {
    let mut second = phases::Second::new(first);
    let mut associables = ActiveAssociables::new();
    naming::generate_naming(&mut second.book_accounts);

    first.ledger.iter().try_for_each(|(ledger_id, event)| add_entry(IncompleteParams{
        first: &first,
        second: &mut second,
        ledger_id: *ledger_id,
        associables: &mut associables
    }, event))?;
    Ok(second)
}
