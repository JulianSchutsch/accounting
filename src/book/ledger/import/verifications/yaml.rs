use crate::book::*;

pub fn import_yaml_events(ledger: &mut LedgerBuilder, path: &str, settings: &settings::events::Yaml) -> BookResult {
    if !settings.files.iter().any(|e| e.is_match(path)) {
        return Ok(());
    }
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let entries: Vec<Event> = serde_yaml::from_reader(reader)?;
    for entry in entries {
        ledger.add(entry.date(), EventKind::Verification, entry);
    }
    Ok(())
}