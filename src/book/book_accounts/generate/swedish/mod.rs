mod income;
mod invoice;
mod params;
mod ids;
mod naming;

use crate::book::*;

use params::*;

fn add_entry<'p, 'e:'p>(p: IncompleteParams<'_, 'p, '_, '_, '_, '_>, entry: &'e Event) -> BookResult {
    match entry {
        Event::Income(e) => income::add(p.complete_with(e, entry)).map_err(|e| e.extend("Failed to add income")),
        Event::Invoice(e) => invoice::add(p.complete_with(e, entry)).map_err(|e| e.extend("Failed to add invoice")),
    }
}

pub fn generate<'p:'r, 'r>(import: &'p Import) -> BookResult<BookAccounts<'r>> {
    let mut accounts = BookAccounts::new(import.settings.book_currency);
    naming::generate_naming(&mut accounts);
    let mut bank_transaction_consumer = BankTransactionConsumer::new();
    import.ledger.iter().try_for_each(|(ledger_id, event)| add_entry(IncompleteParams{
        ledger_id: *ledger_id,
        accounts: &mut accounts,
        bank_accounts: &import.bank_accounts,
        bank_transaction_consumer: &mut bank_transaction_consumer,
        import: &import
    }, event))?;
    Ok(accounts)
}
