use crate::book::book_result::*;
use crate::book::exchange_rate::ExchangeRates;
use crate::book::types::*;

use crate::book::settings;

pub fn import(exchange_rates: &mut ExchangeRates, settings: &settings::exchange_rates::Riksbank) -> BookResult<> {
    #[derive(serde::Deserialize)]
    struct DateValue {
        date: Date,
        value: f64,
    }
    let file = std::fs::File::open(settings.file.as_str())?;
    let reader = std::io::BufReader::new(file);
    let entries: Vec<DateValue> = serde_yaml::from_reader(reader).map_err(|e| { BookError::new(format!("Failed to deserialize currency rates from file {}", settings.file.as_str())) })?;
    let currency_series: crate::book::exchange_rate::exchange_rates::CurrencySeries = entries.into_iter().map(|e|(e.date, e.value)).collect();
    exchange_rates.series.insert(settings.currency, currency_series);
    Ok(())
}

