use crate::book::*;
use crate::book::book::tools::period_sum;

#[derive(Default)]
pub struct Result{
    // Operating incomes
    pub net_sales: Amount,
    pub other_operating_income: Amount,
    pub sum_operating_income: Amount,
    // Operating costs
    pub other_external_costs: Amount,
    pub employer_costs: Amount,
    pub other_operating_costs: Amount,
    pub sum_operating_costs: Amount,
    // Financial posts
    pub other_financial_assets: Amount,
    pub other_interest_income: Amount,
    pub other_interest_cost: Amount,
    pub sum_financial_posts: Amount,
    // Result
    pub result_before_dispositions: Amount,
    pub result_before_taxes: Amount,
    pub taxes: Amount,
    pub payed_taxes: Amount,
    pub result_after_taxes: Amount,
}

impl Result {
    pub fn table(&self) -> report::table::Table {
        let mut table = report::table::Table::new();
        table.insert_title_row("Result");
        table.insert_str("Net sales"); table.insert_num(self.net_sales); table.new_line();
        table.insert_str("Other operating income"); table.insert_num(self.other_operating_income); table.new_line();
        table.insert_str("Sum operating income"); table.insert_num(self.sum_operating_income); table.new_line();
        table.insert_row_sep();
        table.insert_str("Other external costs"); table.insert_num(self.other_external_costs); table.new_line();
        table.insert_str("Employer costs"); table.insert_num(self.employer_costs); table.new_line();
        table.insert_str("Other operating costs"); table.insert_num(self.other_operating_costs); table.new_line();
        table.insert_str("Sum operating costs"); table.insert_num(self.sum_operating_costs); table.new_line();
        table.insert_row_sep();
        table.insert_str("Other financial assets"); table.insert_num(self.other_financial_assets); table.new_line();
        table.insert_str("Other interest income"); table.insert_num(self.other_interest_income); table.new_line();
        table.insert_str("Other interest cost"); table.insert_num(self.other_interest_cost); table.new_line();
        table.insert_str("Sum financial posts"); table.insert_num(self.sum_financial_posts); table.new_line();
        table.insert_row_sep();
        table.insert_str("Result before taxes"); table.insert_num(self.result_before_taxes); table.new_line();
        table.insert_str("Taxes"); table.insert_num(self.taxes); table.new_line();
        table.insert_str("Payed taxes"); table.insert_num(self.payed_taxes); table.new_line();
        table.insert_str("Result after taxes"); table.insert_num(self.result_after_taxes);table.new_line();
        return table;
    }

    pub fn generate(period: Period, book: &Book) -> BookResult<Result> {
        let net_sales= period_sum(book, period, BookIdRange::num_new(3000,3799))?;
        let other_operating_income= period_sum(book, period, BookIdRange::num_new(3900, 3999))?;

        let sum_operating_income = net_sales+other_operating_income;

        let other_external_costs= period_sum(book, period, BookIdRange::num_new(5000, 6999))?;
        let employer_costs= period_sum(book, period, BookIdRange::num_new(7000, 7699))?;
        let other_operating_costs= period_sum(book, period, BookIdRange::num_new(7960, 7989))?;

        let sum_operating_costs = other_external_costs + employer_costs + other_operating_costs;

        let other_financial_assets= period_sum(book, period, BookIdRange::num_new(8210, 8269))?;
        let other_interest_income= period_sum(book, period, BookIdRange::num_new(8310, 8399))?;
        let other_interest_cost= period_sum(book, period, BookIdRange::num_new(8410, 8499))?;

        let sum_financial_posts = other_financial_assets + other_interest_income + other_interest_cost;

        let result_before_dispositions = sum_operating_income + sum_operating_costs + sum_financial_posts;
        let result_before_taxes = sum_operating_income + sum_operating_costs + sum_financial_posts;

        let taxes = Amount(0.2*result_before_dispositions.0);
        let payed_taxes = period_sum(book, period, BookIdRange::num_new(2510, 2519))?;

        let result_after_taxes = result_before_taxes - taxes;

        Ok(Result{
            net_sales, other_operating_income, sum_operating_income,
            other_external_costs, employer_costs, other_operating_costs, sum_operating_costs,
            other_financial_assets, other_interest_income, other_interest_cost, sum_financial_posts,
            result_before_dispositions,
            result_before_taxes,
            taxes, payed_taxes,
            result_after_taxes
        })

    }
}
