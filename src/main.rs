mod book;

use book::*;

fn process_root_file(path: &str) -> BookResult {
    let first = phases::First::from_root_file(path)?;
    first.ledger.print();
    let second = book_accounts::generate(&first)?;
    second.book_accounts.print();
    let period = Period::from_dates(Date::from_str("2023-01-01")?, Date::from_str("2023-03-01")?);
    let filter = BookAccountsFilterBuilder::new().limit_date(period).build(&second.book_accounts);
    let complete_book = book::report::book_accounts::complete::generate_complete_accounts_table(filter.clone(), &second.book_accounts);
    let accumulated_book = book::AccumulatedBook::calculate(filter);
    let accumulated = book::report::accumulated_book::complete::generate_complete_accounts_table(&accumulated_book, &second.book_accounts);
    complete_book.print();
    accumulated.print();
/*    for year in first.settings.fiscal_years.iter() {
        println!("Deal with fiscal year {}", year.fiscal_year);
        let annual_account = annual_accounts::generate(&first, &second.book_accounts, year.fiscal_year)?;
        annual_account.print();
    }*/
/*    book_accounts::verify::balance::verify(&second.book_accounts)?;
    let c = report::bank_accounts::filtered_transactions::generate(
        &first.bank_accounts,
        BankTransactionsFilterBuilder::new().show_consumed(),
        Some(&second.consumed_bank_transactions));
    c.print();*/
    Ok(())
}

fn main() {
    let path = std::env::args().nth(1).expect("no path to root file given");
    let result = process_root_file(path.as_str());
    if result.is_err() {
        println!("Err={}", result.err().unwrap());
    }
}
