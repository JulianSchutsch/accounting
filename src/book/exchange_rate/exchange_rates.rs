use crate::book::*;

pub type CurrencySeries = std::collections::BTreeMap<Date, f64>;
type Series = std::collections::HashMap<Currency, CurrencySeries>;

pub struct ExchangeRates {
    book_currency: Currency,
    pub series: Series,
}

impl ExchangeRates {
    pub fn new(book_currency: Currency) -> ExchangeRates {
        ExchangeRates {
            book_currency,
            series: Series::new()
        }
    }

    fn into_book(&self, date: Date, currency: Currency, amount: f64) -> BookResult<f64> {
        if currency==self.book_currency {
            return Ok(amount);
        }
        let currency_series = self.series.get(&currency).ok_or_else(|| BookError::new(format!("Failed to retrieve exchange_rates {}", currency)))?;
        let exchange_rate = currency_series.get(&date).ok_or_else(|| BookError::new("Failed to find exchange rate for date"))?;
        let book_amount = amount*exchange_rate;
        Ok(book_amount)
    }

    pub fn amount_into_book(&self, date: Date, currency: Currency, amount: Amount) -> BookResult<Amount> {
        Ok(Amount(self.into_book(date, currency, amount.0)?))
    }

    pub fn moms_into_book(&self, date: Date, currency: Currency, amount: Amount) -> BookResult<Amount> {
        Ok(Amount(self.into_book(date, currency, amount.0)?))
    }
}
