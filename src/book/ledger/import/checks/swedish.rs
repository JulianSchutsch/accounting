use crate::book::*;

pub fn generate_fiscal_year(ledger: &mut LedgerBuilder, settings: &settings::FiscalYear) -> BookResult {
    for month in settings.fiscal_year.iterate_months() {
        ledger.add(month.end, EventKind::Check, Event::VerifyMoms(VerifyMoms{id: "Verify Moms".to_string(), date: month.end}));
        ledger.add(month.end, EventKind::Check, Event::Check(CheckPeriod{id: "Check".to_string(), period: month}));
    }
    ledger.add(settings.fiscal_year.end, EventKind::Check, Event::EndFiscalYear(EndFiscalYear{id: "End Fiscal Year".to_string(), fiscal_year: settings.fiscal_year}));
    Ok(())
}