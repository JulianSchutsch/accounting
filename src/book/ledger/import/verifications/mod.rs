use crate::book::*;

mod yaml;

fn import_events(ledger: &mut Ledger, path: &str, fiscal_year_settings: &settings::FiscalYear) -> BookResult<> {
    for event_filter in fiscal_year_settings.events.iter() {
        match event_filter {
            settings::events::Events::Yaml(e) => yaml::import_yaml_events(ledger, path, e)?
        }
    }
    Ok(())
}

pub fn import_fiscal_year(mut ledger: &mut Ledger, fiscal_year_settings: &settings::FiscalYear) -> BookResult {
    utils::paths::directory_scan(std::path::Path::new(fiscal_year_settings.root_path.as_str()), &mut |path|{
        import_events(&mut ledger, path, fiscal_year_settings).map_err(|e| e.extend(format!("Failed to import file {}", path)))
    })
}
