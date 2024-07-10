use std::io::BufRead;
use crate::book::*;

pub struct Content {

}

impl Content {
    fn read_period_part(s: &str) -> Option<(String, Date)> {
        let description_regex = regex::Regex::new(r"; (?<desc>\w*) saldo (?<date>\d{4}-\d{2}-\d{2});\d*;\d*")?;
        if let Ok(results) = description_regex.captures(s.as_str()).ok_or_else(|| BookError::new("CSV header not as expected, failed parsing period")) {
            Some((results["desc"].to_string(), Date::from_str(&results["date"])))
        }
        None
    }

    pub fn import(path: &str) -> BookResult<Self> {
        let file = std::fs::File::open(path)?;
        let mut reader = std::io::BufReader::new(file);
        let mut line: String;
        let mut begin: Option<Date>;
        let mut end: Option<Date>;
        while let Some(_) = reader.read_line(&mut line) {
            if let Some((description, date)) = Self::read_period_part(line.as_str()) {
                match description.as_str() {
                    "Ingående" => begin = Some(date),
                    "Utgående" => end = Some(date),
                    _ => ()
                }
            }
        }
        if let None = begin {
            return Err(BookError::new("There is no beginning date"));
        }
        if let None = end {
            return Err(BookError::new("There is no ending date"));
        }
        Ok(Self {})
    }
}