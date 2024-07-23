mod book;

use crate::book::*;

fn display_period(period: Period, first: &phases::First, second: &phases::Second) {
    let filter = BookAccountsFilterBuilder::new().limit_date(period).build(&second.book);
    let complete_book = book::report::book_accounts::complete::generate_complete_accounts_table(filter.clone(), &second.book);
    let accumulated_book = book::AccumulatedBook::calculate(filter);
    let accumulated = book::report::accumulated_book::complete::generate_complete_accounts_table(&accumulated_book, &second.book);
    complete_book.print();
    accumulated.print();
}

fn process_root_file(path: &str) -> BookResult {
    let first = phases::First::from_root_file(path)?;
    let second = book::book::generate::generate(&first)?;
    for fiscal_year in first.settings.fiscal_years.iter() {
        for month in fiscal_year.fiscal_year.iterate_months() {
            display_period(month, &first, &second);
        }
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
