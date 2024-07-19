mod book;

use book::*;

fn process_root_file(path: &str) -> BookResult {
    let first = phases::First::from_root_file(path)?;
    let second = book_accounts::generate(&first)?;
    let period = Period::from_dates(Date::from_str("2023-01-01")?, Date::from_str("2023-04-01")?);
    let filter = BookAccountsFilterBuilder::new().limit_date(period).build(&second.book_accounts);
    let complete_book = book::report::book_accounts::complete::generate_complete_accounts_table(filter.clone(), &second.book_accounts);
    let accumulated_book = book::AccumulatedBook::calculate(filter);
    let accumulated = book::report::accumulated_book::complete::generate_complete_accounts_table(&accumulated_book, &second.book_accounts);
    complete_book.print();
    accumulated.print();
    Ok(())
}

fn main() {
    let path = std::env::args().nth(1).expect("no path to root file given");
    let result = process_root_file(path.as_str());
    if result.is_err() {
        println!("Err={}", result.err().unwrap());
    }
}
