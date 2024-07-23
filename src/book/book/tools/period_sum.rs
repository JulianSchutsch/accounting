use crate::book::*;

pub fn period_sum(book: &Book, period: Period, id_range: BookAccountIdRange, side: BookAccountSide) -> BookResult<Amount> {
    let filter = BookAccountsFilterBuilder::new()
        .limit_id(id_range)
        .limit_date(period)
        .limit_side(side).build(book);
    let mut result = Amount(0.0);
    for ((_date, _ledger_id), entries) in filter {
        for &entry in entries.iter() {
            result += entry.amount.signed_amount();
        }
    }
    Ok(result)
}