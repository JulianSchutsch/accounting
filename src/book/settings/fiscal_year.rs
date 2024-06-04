use crate::book::bookresult::*;
use crate::book::types;
use crate::book::utils::paths::build_path;

use super::banks;
use super::events;

#[derive(serde::Deserialize)]
struct PlainFiscalYear {
    pub fiscal_year: types::FiscalYear,
    pub banks: Vec<banks::PlainBanks>,
    pub events: Vec<events::PlainEvents>
}

pub struct FiscalYear {
    pub root_path: String,
    pub fiscal_year: types::FiscalYear,
    pub banks: Vec<banks::Banks>,
    pub events: Vec<events::Events>
}

impl FiscalYear {

    fn read_plain(path: &String) -> BookResult<PlainFiscalYear> {
        let file = std::fs::File::open(&path).map_err(|e|BookError::new(format!("Failed opening fiscal year settings file {}", &path)))?;
        let reader = std::io::BufReader::new(file);
        let plain: PlainFiscalYear = serde_yaml::from_reader(reader).map_err(|e| BookError::new_from(e, format!("Failed parsing fiscal year settings in file {}", &path)))?;
        Ok(plain)
    }

    pub fn read_from_file(root_path: &String, path: &String) -> BookResult<FiscalYear> {
        let plain = Self::read_plain(&build_path!(root_path, path, "book.yaml"))?;
        Ok(FiscalYear{
            root_path: build_path!(root_path, path),
            fiscal_year: plain.fiscal_year,
            banks: plain.banks.iter().map(|p| { p.to_banks() }).collect::<Result<Vec<_>, _>>()?,
            events: plain.events.iter().map(|p|{ p.to_events() }).collect::<Result<Vec<_>, _>>()?,
        })
    }
}