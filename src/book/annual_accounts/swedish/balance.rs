use crate::book::*;
use crate::book::book::tools::period_sum;

use super::result::Result;

#[derive(Default)]
pub struct Assets {
    // Assets
    pub customer_receivables: Amount,
    pub other_receivables: Amount,
    pub prepaid_costs: Amount,
    pub sum_short_term_receivables: Amount,
    pub cash_and_bank_balances: Amount,
    pub sum_assets: Amount,
}

impl Assets {
    pub fn table(&self) -> report::table::Table {
        let mut table = report::table::Table::new();
        table.insert_title_row("Balance");
        table.insert_str("Customer receivables");
        table.insert_num(self.customer_receivables);
        table.new_line();
        table.insert_str("Other receivables");
        table.insert_num(self.other_receivables);
        table.new_line();
        table.insert_str("Prepaid costs");
        table.insert_num(self.prepaid_costs);
        table.new_line();
        table.insert_str("Sum short term receivables");
        table.insert_num(self.sum_short_term_receivables);
        table.new_line();
        table.insert_str("Cash and bank balances");
        table.insert_num(self.cash_and_bank_balances);
        table.new_line();
        table.insert_str("Sum assets");
        table.insert_num(self.sum_assets);
        table.new_line();
        table.insert_row_sep();
        table
    }

    pub fn generate(period: Period, book: &Book) -> BookResult<Assets> {
        let customer_receivables =
            period_sum(book, period, BookIdRange::num_new(1510, 1539))?
                + period_sum(book, period, BookIdRange::num_new(1550, 1559))?
                + period_sum(book, period, BookIdRange::num_new(1580, 1589))?;
        let other_receivables =
            period_sum(book, period, BookIdRange::num_new(1610, 1619))?
                + period_sum(book, period, BookIdRange::num_new(1630, 1659))?
                + period_sum(book, period, BookIdRange::num_new(1680, 1689))?;
        let prepaid_costs = period_sum(book, period, BookIdRange::num_new(1710, 1799))?;
        let sum_short_term_receivables = customer_receivables + other_receivables + prepaid_costs;
        let cash_and_bank_balances = period_sum(book, period, BookIdRange::num_new(1910, 1989))?;
        let sum_assets = sum_short_term_receivables + cash_and_bank_balances;
        Ok(Assets{
            customer_receivables, other_receivables, prepaid_costs, sum_short_term_receivables,
            cash_and_bank_balances,
            sum_assets
        })
    }

}

#[derive(Default)]
pub struct Equity {
    // Equity capital and debt
    pub share_capital: Amount,
    pub sum_restricted_equity: Amount,
    pub balanced_result: Amount,
    pub years_result: Amount,
    pub sum_unrestricted_equity: Amount,
    pub sum_equity: Amount,
    pub other_long_term_liabilities: Amount,
    pub sum_long_term_liabilities: Amount,
    pub accounts_payable: Amount,
    pub tax_debt: Amount,
    pub other_short_term_liabilities:Amount,
    pub sum_short_term_liabilities: Amount,
    pub sum_assets_and_liabilities: Amount
}

impl Equity {
    pub fn generate(period: Period, book: &Book, previous: &Equity, result: &Result) -> BookResult<Equity> {
        let share_capital = period_sum(book, period, BookIdRange::num_new(2081, 2081))?;
        let sum_restricted_equity = share_capital;
        let balanced_result = previous.balanced_result + result.result_after_taxes;
        let years_result = result.result_after_taxes;
        let sum_unrestricted_equity = balanced_result + years_result;
        let sum_equity = sum_restricted_equity + sum_unrestricted_equity;
        let other_long_term_liabilities = period_sum(book, period,BookIdRange::num_new(2390, 2399))?;
        let sum_long_term_liabilities = other_long_term_liabilities;
        let accounts_payable = period_sum(book, period, BookIdRange::num_new(2440, 2449))?;
        let tax_debt = result.taxes - result.payed_taxes;
        let other_short_term_liabilities =
                period_sum(book, period, BookIdRange::num_new(2499, 2499))?
            +   period_sum(book, period, BookIdRange::num_new(2610, 2669))?
            +   period_sum(book, period, BookIdRange::num_new(2710, 2799))?
            +   period_sum(book, period, BookIdRange::num_new(2810, 2899))?;
        let sum_short_term_liabilities = accounts_payable + tax_debt + other_short_term_liabilities;
        let sum_assets_and_liabilities = sum_equity + sum_long_term_liabilities+ sum_short_term_liabilities;
        Ok(Equity{
            share_capital, sum_restricted_equity,
            balanced_result, years_result, sum_unrestricted_equity,
            sum_equity,
            other_long_term_liabilities,
            sum_long_term_liabilities,
            accounts_payable, tax_debt, other_short_term_liabilities, sum_short_term_liabilities,
            sum_assets_and_liabilities
        })
    }

    pub fn table(&self) -> report::table::Table {
        let mut table = report::table::Table::new();
        table.insert_title_row("Equity and liabilities");
        table.insert_str("Share capital");
        table.insert_num(self.share_capital);
        table.new_line();
        table.insert_str("Sum restricted equity");
        table.insert_num(self.sum_restricted_equity);
        table.new_line();
        table.insert_str("Balanced result");
        table.insert_num(self.balanced_result);
        table.new_line();
        table.insert_str("Years result");
        table.insert_num(self.years_result);
        table.new_line();
        table.insert_str("Sum unrestricted equity");
        table.insert_num(self.sum_unrestricted_equity);
        table.new_line();
        table.insert_str("Other long term liabilities");
        table.insert_num(self.other_long_term_liabilities);
        table.new_line();
        table.insert_str("Sum long term liabilities");
        table.insert_num(self.sum_long_term_liabilities);
        table.new_line();
        table.insert_str("Accounts payable");
        table.insert_num(self.accounts_payable);
        table.new_line();
        table.insert_str("Tax debt");
        table.insert_num(self.tax_debt);
        table.new_line();
        table.insert_str("Other short term liabilities");
        table.insert_num(self.other_short_term_liabilities);
        table.new_line();
        table.insert_str("Sum short term liabilities");
        table.insert_num(self.sum_short_term_liabilities);
        table.new_line();
        table.insert_str("Sum assets and liabilities");
        table.insert_num(self.sum_assets_and_liabilities);
        table.new_line();
        table
    }
}

#[derive(Default)]
pub struct Balance {
    pub assets: Assets,
    pub equity: Equity
}

impl Balance {
    pub fn generate(period: Period, book: &Book, previous: &Balance, result: &Result) -> BookResult<Balance> {
        Ok(Balance{
            assets: Assets::generate(period, book)?,
            equity: Equity::generate(period, book, &previous.equity, result)?
        })
    }
}