mod book;

use book::*;

fn process_root_file(path: &str) -> BookResult {
    let first = phases::First::from_root_file(path)?;
    first.ledger.print();
    let second = book_accounts::generate(&first)?;
    second.book_accounts.print();
    let complete_book = book::report::book_accounts::complete::generate_complete_accounts_table(&second.book_accounts);
    complete_book.print();
    for year in first.settings.fiscal_years.iter() {
        println!("Deal with fiscal year {}", year.fiscal_year);
        let annual_account = annual_accounts::generate(&first, &second.book_accounts, year.fiscal_year)?;
        annual_account.print();
    }
    book_accounts::verify::balance::verify(&second.book_accounts)?;
    let c = report::bank_accounts::filtered_transactions::generate(
        &first.bank_accounts,
        BankTransactionsFilterBuilder::new(),
        Some(&second.consumed_bank_transactions));
    c.print();
    Ok(())
}

fn main() {
    let path = std::env::args().nth(1).expect("no path to root file given");
    let result = process_root_file(path.as_str());
    if result.is_err() {
        println!("Err={}", result.err().unwrap());
    }
}
