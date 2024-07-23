use crate::book::*;

pub fn generate_fiscal_year(ledger: &mut Ledger, settings: &settings::FiscalYear) -> BookResult {
    for month in settings.fiscal_year.iterate_months() {
        ledger.events.insert(ledger.ledger_id.generate_time_id(month.end), Event::VerifyMoms(VerifyMoms{id: "".to_string(), date: month.end}));
    }
    ledger.events.insert(ledger.ledger_id.generate_time_id(settings.fiscal_year.end), Event::EndFiscalYear(EndFiscalYear{id: "".to_string(), date: settings.fiscal_year.end}));
    Ok(())
}