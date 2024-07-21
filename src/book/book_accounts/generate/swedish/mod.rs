mod income;
mod invoice;
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
mod bank_cost;
mod exchange;
mod params;
mod payment;

use crate::book::*;

use params::*;
use crate::book::book_accounts::generate::swedish::active_associables::ActiveAssociables;

fn add_entry<'p>(ledger_id: LedgerId, entry: &Event, params: & mut Params<'p>, associables: &mut ActiveAssociables<'p>) -> BookResult {
    match entry {
        Event::Income(e) => income::add(ledger_id, e, params, associables).map_err(|e| e.extend("Failed to add income")),
        Event::Invoice(e) => invoice::add(ledger_id, e, params).map_err(|e| e.extend("Failed to add invoice")),
        Event::Salary(e) => salary::add(ledger_id, e, params).map_err(|e| e.extend("Failed to add salary")),
        Event::TaxPayment(e) => tax_payment::add(ledger_id, e, params).map_err(|e| e.extend("Failed to add tax payment")),
        Event::Interest(e) => interest::add(ledger_id, e, params).map_err(|e| e.extend("Failed to add interest")),
        Event::Fine(e) => fine::add(ledger_id, e, params).map_err(|e| e.extend("Failed to add fine")),
        Event::VerifyMoms(e) => verify_moms::add(ledger_id, e, params).map_err(|e| e.extend("Failed to add verify moms")),
        Event::Shares(e) => shares::add(ledger_id, e, params, associables).map_err(|e| e.extend("Failed to add shares")),
        Event::Transaction(e) => transaction::add(ledger_id, e, params, associables).map_err(|e| e.extend("Failed to add transaction")),
        Event::BankCost(e) => bank_cost::add(ledger_id, e, params).map_err(|e| e.extend("Failed to add bank cost")),
        Event::Exchange(e) => exchange::add(ledger_id, e, params, associables).map_err(|e| e.extend("Failed to add exchange")),
    }
}

pub fn generate(first: &phases::First) -> BookResult<phases::Second> {

    let mut params = Params::new(first);
    naming::generate_naming(&mut params.book);
    let mut associables = active_associables::ActiveAssociables::new();
    for (ledger_id, event) in first.ledger.iter() {
        add_entry(*ledger_id, event, &mut params, &mut associables)?;
    }

    Ok(phases::Second::new(first, params.book))
}
