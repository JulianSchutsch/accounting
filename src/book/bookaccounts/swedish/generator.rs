mod params;
mod income;
mod invoice;

use crate::book::bookresult::*;
use crate::book::bookaccounts::*;
use crate::book::ledger::*;
use crate::book::converter::Converter;

use params::*;

use super::methods::Method;

fn add_entry<'p, 'e:'p>(p: IncompleteParams<'_, 'p, '_>, entry: &'e Event) -> BookResult {
    match entry {
        Event::Income(e) => income::add(p.complete_with(e, entry)).map_err(|e| e.extend_by_str("Failed to add income"))?,
        Event::Invoice(e) => invoice::add(p.complete_with(e, entry)).map_err(|e| e.extend_by_str("Failed to add invoice"))?,
    }
    Ok(())
}

fn generate<'p:'r, 'r>(converter: &Converter, &ledger: &'p Ledger) -> BookResult<Accounts<'r>> {
    let mut accounts = Accounts::new(converter.book_currency());
    ledger.iter().try_for_each(|(ledger_id, e)| add_entry(IncompleteParams{
        ledger_id: *ledger_id,
        accounts: &mut accounts,
        converter: converter,
    }, e))?;
    Ok(accounts)
}
