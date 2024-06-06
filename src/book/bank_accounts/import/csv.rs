use std::collections::HashSet;
use std::io::BufRead;
use crate::book::bank_accounts::bank_account_reference::{BankAccountReference, SwedishAccountNumber};
use crate::book::bank_accounts::bank_accounts::BankAccountType;
use crate::book::bank_accounts::BankAccounts;
use crate::book::book_result::*;
use crate::book::types::*;
use crate::book::settings;

#[derive(Debug, serde::Deserialize)]
struct Row {
    #[serde(rename="Radnr")]
    _row_number: i32,
    #[serde(rename="Clnr")]
    _clearing_number: i32,
    #[serde(rename="Kontonr")]
    account_nr: i64,
    #[serde(rename="Produkt")]
    _product: String,
    #[serde(rename="Valuta")]
    _currency: Currency,
    #[serde(rename="Bokfdag")]
    executed_date: Date,
    #[serde(rename="Transdag")]
    transaction_date: Date,
    #[serde(rename="Valutadag")]
    value_date: Date,
    #[serde(rename="Referens")]
    reference: String,
    #[serde(rename="Text")]
    text: String,
    #[serde(rename="Belopp")]
    amount: f32,
    #[serde(rename="Saldo")]
    accumulated: f32
}

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

pub fn import(banks : &mut BankAccounts, path: &str, settings: &settings::banks::CSV) -> BookResult<()> {
    if !settings.files.iter().any(|e| e.is_match(path)) {
        return Ok(());
    }
    let file = encoding_rs_io::DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding_rs::WINDOWS_1252))
        .build(std::fs::File::open(path)?);
    let mut reader = std::io::BufReader::new(file);
    let period = read_header(&mut reader)?;
    let mut csv_reader = csv::Reader::from_reader(reader);
    let rows = csv_reader.deserialize().collect::<Result<Vec<Row>,_>>()?;
    let accounts = rows.iter().map(|row| row.account_nr).collect::<HashSet<i64>>();
    for account in accounts {
        let ref1 = BankAccountReference::SwedishAccountNumber(SwedishAccountNumber{number: account});
        let account_id = banks.ensure_account(
            HashSet::from_iter(vec![ref1].into_iter()),
            BankAccountType::Account, None, None
        )?;
        banks.add_period(account_id, period, path.to_string());
    }
    Ok(())
}