use crate::book::bookresult::BookResult;
use crate::book::exchange_rate::ExchangeRates;
use crate::book::types::*;

use crate::book::settings;

fn import_file(converter: &mut ExchangeRates, path: &String) -> BookResult<> {
    #[derive(serde::Deserialize)]
    struct DateValue {
        date: Date,
        value: f64,
    }
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let entries: Vec<DateValue> = serde_yaml::from_reader(reader).unwrap();
    let currency_series: crate::book::exchange_rate::exchange_rates::CurrencySeries = entries.iter().map(|e|(e.date, e.value)).collect();
    converter.series.insert(converter.book_currency, currency_series);
    Ok(())
}

pub fn import(converter: &mut ExchangeRates, settings: &settings::exchange_rates::Riksbank) -> BookResult<> {
    for path in settings.files.iter() {
        import_file(converter, path)?;
    }
    Ok(())
}

