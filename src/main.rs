mod book;

use crate::book::*;

fn display_banks(period: Period, first: &phases::First) {
    for (_, account) in first.bank_accounts.iter() {
        let first = account.latest_value(period.begin);
        let last =  account.latest_value(period.end);
        if let Some(first_value) = first {
            if let Some(last_value) = last {
                println!("Acocunt {}, Values during {} are from {}..{} {}", account.references, period, first_value, last_value, account.currency);
            } else {
                println!("Account {} missing last value", account.references);
            }
        } else {
            println!("Account {} missing first value", account.references);
        }
    }
    for currency in ALL_CURRENCIES {
        let b = first.bank_accounts.sum_latest_values(currency, period.begin);
        let e = first.bank_accounts.sum_latest_values(currency, period.end);
        println!("Currency total {} -> {} {}", b, e, currency);
    }
}

fn display_period(period: Period, fiscal_year: Period, first: &phases::First, second: &phases::Second) {
    let filter = BookFilterBuilder::new().limit_date(period).build(&second.book);
    let filter_from_start = BookFilterBuilder::new().limit_date(Period::from_dates(fiscal_year.begin, period.end)).build(&second.book);
    let complete_book = book::report::book_accounts::complete::generate_complete_accounts_table(filter.clone(), &second.book);
    let accumulated_book = book::AccumulatedBook::calculate(filter_from_start);
    let accumulated = book::report::accumulated_book::complete::generate_complete_accounts_table(&accumulated_book, &second.book);
    complete_book.print();
    accumulated.print();
    println!("Accumulated total: {}", accumulated_book.total);
}

fn process_root_file(path: &str) -> BookResult {
    let first = phases::First::from_root_file(path)?;
    let second = book::book::generate::generate(&first)?;
    for fiscal_year in first.settings.fiscal_years.iter() {
        for month in fiscal_year.fiscal_year.iterate_months() {
            display_period(month, fiscal_year.fiscal_year, &first, &second);
        }
        display_banks(fiscal_year.fiscal_year, &first);
        let previous_k2 = annual_accounts::swedish::K2::new();
        let k2 = annual_accounts::swedish::K2::generate(&first, &fiscal_year, &second.book, &previous_k2)?;
        k2.result.table().print();
        k2.balance.assets.table().print();
        k2.balance.equity.table().print();
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
