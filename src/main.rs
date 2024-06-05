mod book;

use book::BookResult;
use book::Import;
use book::book_accounts;

fn process_root_file(path: &str) -> BookResult {
    let import = Import::from_root_file(path)?;
    import.ledger.print();
    let book_accounts = book_accounts::generate(&import)?;
    book_accounts.print();
    let complete_book = book::report::bookaccounts::complete::generate_complete_accounts_table(&book_accounts);
    complete_book.print();
    Ok(())
}

fn main() {
    let path = std::env::args().nth(1).expect("no path to root file given");
    let result = process_root_file(path.as_str());
    if result.is_err() {
        println!("{}", result.err().unwrap());
    }
}
