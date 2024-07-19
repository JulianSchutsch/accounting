use crate::book::*;

pub fn import_yaml_events(ledger: &mut Ledger, path: &str, settings: &settings::events::Yaml) -> BookResult {
    if !settings.files.iter().any(|e| e.is_match(path)) {
        return Ok(());
    }
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let entries: Vec<Event> = serde_yaml::from_reader(reader)?;
    for entry in entries {
        ledger.events.insert(ledger.ledger_id.generate_verification_id(), entry);
    }
    Ok(())
}