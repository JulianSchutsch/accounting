use crate::book::types::*;
use crate::book::bookresult::BookResult;

type CurrencySeries = std::collections::BTreeMap<Date, f64>;
type Series = std::collections::HashMap<Currency, CurrencySeries>;

pub struct Converter {
    book_currency: Currency,
    series: Series,
}

impl Converter {
    pub fn new(book_currency: Currency) -> Converter {
        Converter{
            book_currency: book_currency,
            series: Series::new()
        }
    }
    
    pub fn add_riksbank_series(&mut self, path: String, currency: Currency) -> BookResult<()> {
        #[derive(serde::Deserialize)]
        struct DateValue {
            date: Date,
            value: f64,
        }
        let file = std::fs::File::open(path).map_err(|_| "Failed to open path")?;
        let reader = std::io::BufReader::new(file);
        let entries: Vec<DateValue> = serde_yaml::from_reader(reader).unwrap();
        let currency_series: CurrencySeries = entries.iter().map(|e|(e.date, e.value)).collect();
        self.series.insert(currency, currency_series);
        Ok(())
    }
    pub fn into_book(&self, date: Date, currency: Currency, amount: f64) -> BookResult<f64> {
        if currency==self.book_currency {
            return Ok(amount);
        }
        let currency_series = self.series.get(&currency).ok_or_else(|| "Failed to retrieve currency")?;
        let exchange_rate = currency_series.get(&date).ok_or_else(|| "Failed to find exchange rate for date")?;
        let bookamount = amount*exchange_rate;
        Ok(bookamount)
    }
}

impl crate::book::converter::Converter for Converter {
    fn book_currency(&self) -> Currency {
        return self.book_currency;
    }
    fn amount_into_book(&self, date: Date, currency: Currency, amount: Amount) -> BookResult<BookAmount> {
        Ok(BookAmount(self.into_book(date, currency, amount.0)?))
    }
    fn moms_into_book(&self, date: Date, currency: Currency, amount: Moms) -> BookResult<BookAmount> {
        Ok(BookAmount(self.into_book(date, currency, amount.0)?))
    }
}
