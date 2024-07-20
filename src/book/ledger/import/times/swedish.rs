use crate::book::*;

pub fn generate_fiscal_year(ledger: &mut Ledger, settings: &settings::FiscalYear) -> BookResult {
    let mut verify_moms_day = settings.fiscal_year.begin.last_day_this_month()?;
    loop {
        ledger.events.insert(ledger.ledger_id.generate_time_id(verify_moms_day), Event::VerifyMoms(VerifyMoms{id: "".to_string(), date: verify_moms_day}));
        if verify_moms_day>=settings.fiscal_year.end {
            break;
        }
        verify_moms_day = verify_moms_day.last_day_next_month()?;
    }
    Ok(())
}