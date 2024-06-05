mod book;

use book::BookResult;
use book::Import;
use book::book_accounts;
use book::annual_accounts;

fn process_root_file(path: &str) -> BookResult {
    let import = Import::from_root_file(path)?;
    import.ledger.print();
    let book_accounts = book_accounts::generate(&import)?;
    book_accounts.print();
    let complete_book = book::report::bookaccounts::complete::generate_complete_accounts_table(&book_accounts);
    complete_book.print();
    for year in import.settings.fiscal_years.iter() {
        println!("Deal with fiscal year {}", year.fiscal_year);
        let annual_account = annual_accounts::generate(&import, &book_accounts, year.fiscal_year)?;
        annual_account.print();
    }
    Ok(())
}

fn main() {
    let path = std::env::args().nth(1).expect("no path to root file given");
    let result = process_root_file(path.as_str());
    if result.is_err() {
        println!("Err={}", result.err().unwrap());
    }
}
