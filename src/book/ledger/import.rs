mod yaml;

use crate::book::bookresult::*;
use crate::book::settings;
use crate::book::utils::paths::*;
use super::ledger::{Ledger, LedgerId};

fn import_events(ledger: &mut Ledger, ledger_id: &mut LedgerId, path: &str, fiscal_year_settings: &settings::FiscalYear) -> BookResult<> {
    for event_filter in fiscal_year_settings.events.iter() {
        match event_filter {
            settings::events::Events::Yaml(e) => yaml::import_yaml_events(ledger, ledger_id, path, e)?
        }
    }
    Ok(())
}

fn import_fiscal_year(mut ledger: &mut Ledger, fiscal_year_settings: &settings::FiscalYear) -> BookResult<> {
    let mut ledger_id: LedgerId = LedgerId::initial(fiscal_year_settings.fiscal_year.begin);
    directory_scan(std::path::Path::new(fiscal_year_settings.root_path.as_str()), &mut |path|{
        import_events(&mut ledger, &mut ledger_id, path, fiscal_year_settings)
    })
}

pub fn import_using_settings(settings: &settings::Settings) -> BookResult<Ledger> {
    let mut ledger = Ledger::new();
    for fiscal_year in settings.fiscal_years.iter() {
        import_fiscal_year(&mut ledger, fiscal_year)?;
    }
    Ok(ledger)
}