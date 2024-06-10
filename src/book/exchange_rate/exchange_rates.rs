use crate::book::*;

pub type CurrencySeries = std::collections::BTreeMap<Date, f64>;
type Series = std::collections::HashMap<Currency, CurrencySeries>;

pub struct ExchangeRates {
    pub book_currency: Currency,
    pub series: Series,
}

impl ExchangeRates {
    pub fn new(book_currency: Currency) -> ExchangeRates {
        ExchangeRates {
            book_currency,
            series: Series::new()
        }
    }

    pub fn convert_into_book_currency(&self, date: Date, currency: Currency, amount: Amount) -> BookResult<Amount> {
        if currency==self.book_currency {
            return Ok(amount);
        }
        let currency_series = self.series.get(&currency).ok_or_else(|| BookError::new(format!("Failed to retrieve exchange_rates {}", currency)))?;
        let exchange_rate = currency_series.get(&date).ok_or_else(|| BookError::new("Failed to find exchange rate for date"))?;
        let book_amount = Amount(amount.0 * exchange_rate);
        Ok(book_amount)
    }

}
