use crate::book::*;

pub fn import_yaml_events(ledger: &mut Ledger, ledger_id: &mut LedgerId, path: &str, settings: &settings::events::Yaml) -> BookResult {
    #[derive(serde::Deserialize)]
    enum DeEvent {
        #[serde(rename="income")]
        Income(Income),
        #[serde(rename="invoice")]
        Invoice(Invoice),
        #[serde(rename="salary")]
        Salary(Salary),
        #[serde(rename="shares")]
        Shares(Shares)
    }
    if !settings.files.iter().any(|e| e.is_match(path)) {
        return Ok(());
    }
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let entries: Vec<DeEvent> = serde_yaml::from_reader(reader)?;
    for entry in entries {
        match entry {
            DeEvent::Income(e) =>  {
                ledger.events.insert(ledger_id.clone(), Event::Income(e));
                ledger_id.increase();
            },
            DeEvent::Invoice(e) => {
                ledger.events.insert(ledger_id.clone(), Event::Invoice(e));
                ledger_id.increase();
            }
            DeEvent::Salary(e) => {
                ledger.events.insert(ledger_id.clone(), Event::Salary(e));
                ledger_id.increase();
            }
            DeEvent::Shares(e) => {
                ledger.events.insert(ledger_id.clone(), Event::Shares(e));
                ledger_id.increase();
            }
        }
    }
    Ok(())
}