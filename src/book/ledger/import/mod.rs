mod yaml;

use crate::book::*;

use super::ledger::*;

fn import_events(ledger: &mut Ledger, ledger_id: &mut LedgerId, path: &str, fiscal_year_settings: &settings::FiscalYear) -> BookResult<> {
    for event_filter in fiscal_year_settings.events.iter() {
        match event_filter {
            settings::events::Events::Yaml(e) => yaml::import_yaml_events(ledger, ledger_id, path, e)?
        }
    }
    Ok(())
}

fn import_fiscal_year(mut ledger: &mut Ledger, fiscal_year_settings: &settings::FiscalYear) -> BookResult {
    let mut ledger_id: LedgerId = LedgerId::initial(fiscal_year_settings.fiscal_year.begin);
    utils::paths::directory_scan(std::path::Path::new(fiscal_year_settings.root_path.as_str()), &mut |path|{
        import_events(&mut ledger, &mut ledger_id, path, fiscal_year_settings)
    })
}

fn verify_and_complete_events(ledger: &mut Ledger) -> BookResult {
    for (_, event) in ledger.events.iter_mut() {
        event.verify_and_complete()?;
    }
    Ok(())
}

pub fn import_using_settings(settings: &settings::Settings) -> BookResult<Ledger> {
    let mut ledger = Ledger::new();
    for fiscal_year in settings.fiscal_years.iter() {
        import_fiscal_year(&mut ledger, fiscal_year)?;
    }
    verify_and_complete_events(&mut ledger)?;
    Ok(ledger)
}