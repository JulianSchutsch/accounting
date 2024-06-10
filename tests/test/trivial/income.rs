use std::collections::BTreeMap;
use book::*;

fn categorized_amounts(total: Amount, category: Category, currency: Currency) -> CategorizedAmounts {
    let mut amounts = BTreeMap::new();
    amounts.insert(category, total);
    CategorizedAmounts { currency, total, reverse_charge: currency==Currency::SEK, amounts }
}

pub fn income(id: String, date: Date, amount: Amount, category: Category) -> Event {
    Event::Income(Income {
        id,
        date,
        country: Country::Sweden,
        customer_vat: "?".to_string(),
        description: "?".to_string(),
        amounts: categorized_amounts(amount, category, Currency::SEK)
    })
}

pub fn world_income(id: String, date: Date, amount: Amount, currency: Currency, category: Category, country: Country) -> Event {
    Event::Income(Income {
        id,
        date,
        country,
        customer_vat: "?".to_string(),
        description: "?".to_string(),
        amounts: categorized_amounts(amount, category, currency)
    })
}
