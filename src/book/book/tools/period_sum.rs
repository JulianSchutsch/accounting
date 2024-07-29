use crate::book::*;

pub fn period_sum(book: &Book, period: Period, id_range: BookIdRange) -> BookResult<Amount> {
    let filter = BookFilterBuilder::new()
        .limit_id(id_range)
        .limit_date(period).build(book);
    let mut result = Amount(0.0);
    for ((_date, _ledger_id), entries) in filter {
        for &entry in entries.iter() {
            result += entry.amount.signed_amount();
        }
    }
    Ok(result)
}

pub fn extract_sums(ledger_id: LedgerId, date: Date, event_id: &String, book: &mut Book, period: Period, id_range: BookIdRange) -> BookResult<Amount> {
    let mut total = Amount::zero();
    for id in id_range.iter() {
        let amount = book::tools::period_sum(
            &book,
            period,
            BookIdRange::single(id),
        )?;
        if !amount.almost_zero() {
            book.add_entry(ledger_id, date,event_id, id, BookAmount::from_signed_amount(-amount));
        }
        total += amount;
    }
    Ok(total)
}
