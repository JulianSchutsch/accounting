pub mod riksbank;

use crate::book::book_result::*;
use crate::book::settings;

use super::exchange_rates::ExchangeRates;

pub fn import_using_settings(settings: &settings::Settings) -> BookResult<ExchangeRates> {
    let mut currency_converter = ExchangeRates::new(settings.book_currency);
    for exchange_rate in settings.exchange_rates.iter() {
        match exchange_rate {
            settings::ExchangeRate::Riksbank(s) => riksbank::import(&mut currency_converter, s)?
        }
    }
    Ok(currency_converter)
}
