mod book;

use crate::book::book::tools::period_sum;
use crate::book::*;
use crate::book::book::generate::swedish::ids;

fn display_banks(period: Period, first: &phases::First, second: &phases::Second) -> BookResult {
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
    let mut total_e = Amount::zero();
    for currency in ALL_CURRENCIES {
        let b = first.bank_accounts.sum_latest_values(currency, period.begin);
        let e = first.bank_accounts.sum_latest_values(currency, period.end);
        println!("Accumulated currency {} -> {} {}", b, e, currency);
        total_e += first.exchange_rates.convert_into_book_currency(period.end, currency, e, None)?;
    }
    println!("Total -> {} {}", total_e, first.exchange_rates.book_currency);
    let debt = -period_sum(&second.book, period.extend_to_min(), BookIdRange::single(ids::DEBT_TO_COMPANY_OWNERS))?;
    let prepaid = period_sum(&second.book, period.extend_to_min(), BookIdRange::single(ids::PRELIMINARY_PAID_COMPANY_TAX))?;
    let shares = -period_sum(&second.book, period.extend_to_min(), BookIdRange::single(ids::SHARES_CAPITAL))?;
    total_e-=debt;
    total_e+=prepaid;
    total_e-=shares;
    println!("Debt to owner {}", debt);
    println!("Prepaid tax {}", prepaid);
    println!("Shares {}", shares);
    println!("Total-debt+taxes-shares {}", total_e);
    Ok(())
}

fn display_period(period: Period, fiscal_year: Period, first: &phases::First, second: &phases::Second) -> BookResult {
    let filter = BookFilterBuilder::new().limit_date(period).build(&second.book);
    let filter_from_start = BookFilterBuilder::new().limit_date(Period::from_dates(fiscal_year.begin, period.end)).build(&second.book);
    let complete_book = book::report::book_accounts::complete::generate_complete_accounts_table(filter.clone(), &second.book);
    let accumulated_book = book::AccumulatedBook::calculate(filter_from_start);
    let accumulated = book::report::accumulated_book::complete::generate_complete_accounts_table(&accumulated_book, &second.book);
    complete_book.print();
    accumulated.print();
    println!("Accumulated total: {}", accumulated_book.total);
    display_banks(period, first, second)?;
    Ok(())
}

fn process_root_file(path: &str) -> BookResult {
    let first = phases::First::from_root_file(path)?;
    //first.ledger.print();
    let second = book::book::generate::generate(&first)?;
    for fiscal_year in first.settings.fiscal_years.iter() {
        for month in fiscal_year.fiscal_year.iterate_months() {
            display_period(month, fiscal_year.fiscal_year, &first, &second)?;
        }
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
