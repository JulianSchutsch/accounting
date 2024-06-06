use crate::book::*;

pub fn import_yaml_events(ledger: &mut Ledger, ledger_id: &mut LedgerId, path: &str, settings: &settings::events::Yaml) -> BookResult {
    if !settings.files.iter().any(|e| e.is_match(path)) {
        return Ok(());
    }
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let entries: Vec<Event> = serde_yaml::from_reader(reader)?;
    for entry in entries {
        ledger.events.insert(ledger_id.clone(), entry);
        ledger_id.increase();
    }
    Ok(())
}