use std::io::BufRead;
use std::str::FromStr;
use crate::book::*;

pub struct Row {
    pub date: Date,
    pub description: String,
    pub amount: Amount,
}

pub struct Content {
    pub s_ref: BankAccountReference,
    pub period: Period,
    pub start_value: Amount,
    pub stop_value: Amount,
    pub rows: Vec<Row>
}

lazy_static::lazy_static! {
    static ref description_regex: regex::Regex = regex::Regex::new(r";(?<desc>\w*) saldo (?<date>\d{4}-\d{2}-\d{2});(?<value>\d*);\d*;").unwrap();
    static ref line_regex: regex::Regex = regex::Regex::new(r"(?<date>\d{4}-\d{2}-\d{2});(?<desc>[^;]*);(?<amount>-?\d*);;").unwrap();
    static ref filename_regex: regex::Regex = regex::Regex::new(r"bokf_trans_(?<ref>\d*)\.skv").unwrap();
}

impl Content {
    fn read_period_part(period: &mut Period, start_value: &mut Amount, stop_value: &mut Amount, s: &str) -> bool {
        if let Some(results) = description_regex.captures(s) {
            if let Ok(date) = Date::from_str(&results["date"]) {
                if let Ok(value) = Amount::from_str(&results["value"]) {
                    match &results["desc"] {
                        r"Ingående" => {period.begin = date; *start_value = value; }
                        r"Utgående" => {period.end = date; *stop_value = value; }
                        _ => ()
                    }
                    return true
                }
            }
        }
        false
    }

    fn read_entry(rows: &mut Vec<Row>, s: &str) -> bool {
        if let Some(results) = line_regex.captures(s) {
            if let Ok(date) = Date::from_str(&results["date"]) {
                let amount = Amount(f64::from_str(&results["amount"]).unwrap());
                let description = results["desc"].to_string();
                rows.push(Row {
                    date,
                    description,
                    amount
                });
            }
            return true;
        }
        false
    }

    pub fn extract_ref(path: &str) -> BookResult<BankAccountReference> {
        let filename = std::path::Path::new(path)
            .file_name().ok_or_else(|| BookError::new("Invalid filename to import skv (1)"))?
            .to_str().ok_or_else(|| BookError::new("Invalid filename to import skv (2)"))?;
        let capture = filename_regex.captures(filename).ok_or_else(|| BookError::new("Invalid filename to import skv (regex)"))?;
        let sref = SkatteverketReference{ number: i64::from_str(&capture["ref"])? };
        Ok(BankAccountReference::Skatteverket(sref))
    }

    pub fn import(path: &str) -> BookResult<Self> {
        let s_ref = Self::extract_ref(path)?;

        let file = encoding_rs_io::DecodeReaderBytesBuilder::new()
            .encoding(Some(encoding_rs::WINDOWS_1252))
            .build(std::fs::File::open(path)?);
        let mut reader = std::io::BufReader::new(file);
        let mut line = String::new();

        let mut period: Period = Period::new();
        let mut start_value: Amount = Amount(0.0);
        let mut stop_value: Amount = Amount(0.0);
        let mut rows: Vec<Row> = Vec::new();

        while let Ok(buf) = reader.fill_buf() {
            if buf.is_empty() {
                break;
            }
            reader.read_line(&mut line)?;
            if !Self::read_period_part(&mut period, &mut start_value, &mut stop_value, line.as_str()) {
                if !Self::read_entry(&mut rows, line.as_str()) {
                    return Err(BookError::new(format!("Invalid line in {} = {}", path, line)));
                }
            }
            line.clear();
        }
        Ok(Self{s_ref, period, start_value, stop_value, rows})
    }
}