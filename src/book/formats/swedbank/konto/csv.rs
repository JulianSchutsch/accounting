use crate::book::*;
use std::io::BufRead;

#[derive(Debug, serde::Deserialize)]
pub struct Row {
    #[serde(rename="Radnr")]
    _row_number: i32,
    #[serde(rename="Clnr")]
    _clearing_number: i32,
    #[serde(rename="Kontonr")]
    pub account_nr: i64,
    #[serde(rename="Produkt")]
    _product: String,
    #[serde(rename="Valuta")]
    pub currency: Currency,
    #[serde(rename="Bokfdag")]
    _executed_date: Date,
    #[serde(rename="Transdag")]
    pub transaction_date: Date,
    #[serde(rename="Valutadag")]
    _value_date: Date,
    #[serde(rename="Referens")]
    pub reference: String,
    #[serde(rename="Text")]
    pub text: String,
    #[serde(rename="Belopp")]
    pub amount: Amount,
    #[serde(rename="Saldo")]
    _accumulated: f32
}

pub struct Content {
    pub period: Period,
    pub rows: Vec<Row>
}

impl Content {
    fn read_header<T: std::io::Read>(reader: &mut std::io::BufReader<T>) -> BookResult<Period> {
        let mut description = String::new();
        reader.read_line(&mut description)?;
        let description_regex = regex::Regex::new(r"^\* *Transaktionsrapport *Period *(?<begin>\d{4}-\d{2}-\d{2})[^0-9]+(?<end>\d{4}-\d{2}-\d{2})")?;
        let results = description_regex.captures(description.as_str()).ok_or_else(|| BookError::new("CSV header not as expected, failed parsing period"))?;
        Ok(Period{
            begin: Date::from_str(&results["begin"])?,
            end: Date::from_str(&results["end"])?
        })
    }

    pub fn import(path: &str) -> BookResult<Self> {
        let file = encoding_rs_io::DecodeReaderBytesBuilder::new()
            .encoding(Some(encoding_rs::WINDOWS_1252))
            .build(std::fs::File::open(path)?);
        let mut reader = std::io::BufReader::new(file);
        let period = Self::read_header(&mut reader)?;

        let mut csv_reader = csv::Reader::from_reader(reader);
        let rows = csv_reader.deserialize().collect::<Result<Vec<Row>,_>>()?;
        Ok(Self { period, rows })
    }
}