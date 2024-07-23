use std::collections::BTreeMap;

use crate::book::*;

pub struct AccumulatedBook {
    pub values: BTreeMap<BookId, Amount>
}

impl AccumulatedBook {
    pub fn calculate(filter: BookFilter) -> Self {
        let mut result = Self{
            values: BTreeMap::new()
        };
        for ((_date, _ledger_id), entries) in filter {
            for entry in entries.iter() {
                *result.values.entry(entry.account).or_insert(Amount(0.0)) += entry.amount.signed_amount();
            }
        }
        result
    }
}