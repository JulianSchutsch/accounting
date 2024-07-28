use std::collections::BTreeMap;
use crate::book::*;

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct LedgerBuilderKey {
    date: Date,
    kind: EventKind
}

pub struct LedgerBuilder {
    pub book_currency: Currency,
    events: BTreeMap<LedgerBuilderKey, Vec<Event>>
}

struct LedgerBuilderFiscalYearPos {
    fiscal_year: Period,
    fiscal_year_id: FiscalYearId,
    id: i32
}

impl LedgerBuilderFiscalYearPos {
    pub fn new() -> Self {
        Self{ fiscal_year: Period::new_invalid(), fiscal_year_id: FiscalYearId(0), id: 0}
    }

    pub fn increase(&mut self) {
        self.id += 1;
    }

    pub fn select(date: Date, settings: &settings::Settings) -> BookResult<Self>{
        for fiscal_year in settings.fiscal_years.iter() {
            if fiscal_year.fiscal_year.contains(date) {
                return Ok(LedgerBuilderFiscalYearPos {
                    fiscal_year: fiscal_year.fiscal_year,
                    fiscal_year_id: FiscalYearId(fiscal_year.fiscal_year.begin.id()),
                    id: 0
                });
            }
        }
        Err(BookError::new(format!("No fiscal year for date {}", date)))
    }
    fn valid(&self, date: Date) -> bool {
        self.fiscal_year.contains(date)
    }
}

impl LedgerBuilder {
    pub fn new(book_currency: Currency) -> Self {
        Self { book_currency, events: BTreeMap::new() }
    }


    pub fn build(self, settings: &settings::Settings) -> BookResult<Ledger> {
        let mut ledger = Ledger::new(self.book_currency);
        let mut pos = LedgerBuilderFiscalYearPos::new();
        for (key, events) in self.events.into_iter() {
            if !pos.valid(key.date) {
                pos = LedgerBuilderFiscalYearPos::select(key.date, settings)?;
            }
            let ledger_id = LedgerId::new(pos.fiscal_year_id, pos.id);
            for event in events.into_iter() {
                ledger.events.insert(ledger_id, event);
            }
            pos.increase();
        }
        Ok(ledger)
    }

    pub fn add(&mut self, date: Date, kind: EventKind, event: Event) {
        let key = LedgerBuilderKey { date, kind };
        self.events.entry(key).or_insert(vec![]).push(event);
    }
}