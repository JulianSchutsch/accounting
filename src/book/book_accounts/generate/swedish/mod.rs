mod income;
mod invoice;
mod params;
mod ids;
mod naming;
mod payment;

use crate::book::book_result::*;
use crate::book::import::Import;
use crate::book::book_accounts::BookAccounts;
use crate::book::ledger::Event;

use params::*;

fn add_entry<'p, 'e:'p>(p: IncompleteParams<'_, 'p, '_>, entry: &'e Event) -> BookResult {
    match entry {
        Event::Income(e) => income::add(p.complete_with(e, entry)).map_err(|e| e.extend("Failed to add income")),
        Event::Invoice(e) => invoice::add(p.complete_with(e, entry)).map_err(|e| e.extend("Failed to add invoice")),
        Event::Payment(e) => payment::add(p.complete_with(e, entry)).map_err(|e| e.extend("Failed to add payment"))
    }
}

pub fn generate<'p:'r, 'r>(import: &'p Import) -> BookResult<BookAccounts<'r>> {
    let mut accounts = BookAccounts::new(import.settings.book_currency);
    naming::generate_naming(&mut accounts);
    import.ledger.iter().try_for_each(|(ledger_id, e)| add_entry(IncompleteParams{
        ledger_id: *ledger_id,
        accounts: &mut accounts,
        import: &import
    }, e))?;
    Ok(accounts)
}
