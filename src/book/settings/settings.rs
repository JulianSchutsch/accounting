use crate::book::book_result::*;
use crate::book::types::*;

use super::fiscal_year::FiscalYear;
use super::exchange_rates::ExchangeRate;

#[derive(serde::Deserialize)]
struct PlainSettings {
    pub book_currency: Currency,
    pub book_generator: String,
    pub exchange_rates: Vec<ExchangeRate>,
    pub fiscal_years: Vec<String>,
}

pub struct Settings {
    pub root_directory: String,
    pub book_currency: Currency,
    pub book_generator: String,
    pub exchange_rates: Vec<ExchangeRate>,
    pub fiscal_years: Vec<FiscalYear>,
}

impl Settings {
    fn read_plain(path: &str) -> BookResult<PlainSettings> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let mut plain: Result<PlainSettings, _> = serde_yaml::from_reader(reader);
        plain.map_err(|e| BookError::new(format!("Failed to read settings from {} with error {}", path, e.to_string())))
    }

    fn extract_root_path(path: &str) -> BookResult<String> {
        Ok(
            String::from(std::path::Path::new(&path)
                .parent()
                .ok_or_else(|| BookError::new(format!("No root path can be extracted from given path {}", path)))?
                .canonicalize().map_err(|e| BookError::new(format!("Failed to canonicalize root path {} error {}", path, e.to_string())))?
                .to_str().ok_or_else(|| BookError::new("Unexpected empty root path"))?
            )
        )
    }

    fn read_fiscal_years(plain: &PlainSettings, root_path: &str) -> BookResult<Vec<FiscalYear>> {
        plain.fiscal_years.iter()
            .map(|p| FiscalYear::read_from_file(&root_path, &p))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.extend("Failed reading fiscal years from settings"))
    }

    pub fn read_from_file(path: &str) -> BookResult<Settings> {
        let plain = Self::read_plain(path)?;
        let root_path = Self::extract_root_path(path)?;
        let fiscal_years = Self::read_fiscal_years(&plain, &root_path)?;
        Ok(Settings {
            root_directory: root_path,
            book_currency: plain.book_currency,
            book_generator: plain.book_generator,
            exchange_rates: plain.exchange_rates,
            fiscal_years
        })
    }
}