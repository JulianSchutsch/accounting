use std::collections::BTreeMap;
use std::collections::btree_map::Iter as BTreeMapIter;

use crate::book::*;
use crate::book::settings::ExchangeRate;

fn reverse_charge_default() -> bool { false }

#[derive(Debug, serde::Deserialize)]
pub struct CategorizedAmounts {
    pub currency: Currency,
    pub total : Amount,
    #[serde(default="reverse_charge_default")]
    pub reverse_charge : bool,
    #[serde(flatten)]
    pub amounts: BTreeMap<Category, Amount>,
    pub exchange_rate: Option<f64>
}

impl CategorizedAmounts {
    pub fn iter(&self) -> BTreeMapIter<'_, Category, Amount> {
        return self.amounts.iter();
    }
    pub fn total_diff(&self) -> Amount {
        self.total - self.amounts.iter().fold(Amount(0.0), |acc, (_, v)| acc+*v)
    }

    pub fn convert_into_book_currency(&self, date: Date, exchange_rates: &ExchangeRates) -> BookResult<CategorizedAmounts> {
        Ok(CategorizedAmounts {
            currency: exchange_rates.book_currency,
            total: exchange_rates.convert_into_book_currency(date, self.currency, self.total, self.exchange_rate)?,
            reverse_charge: self.reverse_charge,
            amounts: self.amounts.iter()
                .map(|(&category, amount)| -> BookResult<(Category, Amount)> {
                    let converted = exchange_rates.convert_into_book_currency(date, self.currency, *amount, self.exchange_rate)?;
                    Ok((category, converted))
                }).collect::<Result<BTreeMap<_, _>, _>>()?,
            exchange_rate: None
        })
    }
}
