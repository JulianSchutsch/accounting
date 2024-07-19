use crate::book::*;

pub fn generate(ledger: &mut Ledger, settings: &settings::Settings) -> BookResult {
    for fiscal_year in settings.fiscal_years.iter() {
        let mut ledger_id = LedgerId::post(fiscal_year.fiscal_year.begin);
        let mut verify_moms_day = fiscal_year.fiscal_year.begin.last_day_this_month()?;
        loop {
            ledger.events.insert(ledger_id, Event::VerifyMoms(VerifyMoms{id: "".to_string(), date: verify_moms_day}));
            ledger_id.increase();
            if verify_moms_day>=fiscal_year.fiscal_year.end {
                break;
            }
            verify_moms_day = verify_moms_day.last_day_next_month()?;
        }
    }
    Ok(())
}