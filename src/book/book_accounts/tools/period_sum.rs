use crate::book::*;

pub fn period_sum(book_accounts: &BookAccounts, period: Period, id_range: BookAccountIdRange, side: BookAccountSide) -> BookResult<Amount> {
    let filter = BookAccountsFilterBuilder::new()
        .limit_id(id_range)
        .limit_date(period)
        .limit_side(side).build(book_accounts);
    let mut result = Amount(0.0);
    for ((date, ledger_id), entries) in filter {
        for &entry in entries.iter() {
            result += entry.amount.signed_amount();
        }
    }
    Ok(result)
}