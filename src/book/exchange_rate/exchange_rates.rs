use crate::book::*;

pub type CurrencySeries = std::collections::BTreeMap<Date, f64>;
type Series = std::collections::BTreeMap<Currency, CurrencySeries>;

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

    pub fn convert_into_book_currency(&self, date: Date, currency: Currency, amount: Amount, do_override: Option<f64>) -> BookResult<Amount> {
        if currency==self.book_currency {
            return Ok(amount);
        }
        let currency_series = self.series.get(&currency).ok_or_else(|| BookError::new(format!("Failed to retrieve exchange_rates {}", currency)))?;
        let mut exchange_rate_opt: Option<f64> = do_override;
        if exchange_rate_opt.is_none() {
            for (&d, &v) in currency_series.iter() {
                if d <= date {
                    exchange_rate_opt = Some(v);
                } else {
                    break;
                }
            }
        }
        let exchange_rate = exchange_rate_opt.ok_or_else(|| BookError::new("Failed to find exchange rate for date"))?;
        let unrounded = amount.0 * exchange_rate;
        let rounded = (unrounded*100.0).round()/100.0;
        let book_amount = Amount(rounded);
        Ok(book_amount)
    }

}
