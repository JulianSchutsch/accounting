use std::collections::HashSet;
use std::io::BufRead;

use crate::book::*;

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
    _executed_date: Date,
    #[serde(rename="Transdag")]
    transaction_date: Date,
    #[serde(rename="Valutadag")]
    _value_date: Date,
    #[serde(rename="Referens")]
    reference: String,
    #[serde(rename="Text")]
    _text: String,
    #[serde(rename="Belopp")]
    amount: Amount,
    #[serde(rename="Saldo")]
    _accumulated: f32
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

fn add_account_periods(banks: &mut BankAccounts, rows: &Vec<Row>, period: Period, path: &str) -> BookResult {
    let accounts = rows.iter().map(|row| row.account_nr).collect::<HashSet<i64>>();
    for account in accounts {
        let ref1 = BankAccountReference::SwedishAccountNumber(SwedishAccountNumber{number: account});
        let account = banks.get_mut_account_by_references(&BankAccountReferences::new_from_single(ref1.clone())).ok_or_else(|| BookError::new(format!("Account {} not defined!", ref1)))?;
        account.add_period(period, path.to_string());
    }
    Ok(())
}

pub fn import(banks : &mut BankAccounts, path: &str, settings: &settings::banks::CSV) -> BookResult {
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

    add_account_periods(banks, &rows, period, path)?;

    for row in rows.into_iter() {
        let ref1 = BankAccountReference::SwedishAccountNumber(SwedishAccountNumber{number: row.account_nr});
        banks.get_mut_account_by_references(&BankAccountReferences::new_from_single(ref1.clone()))
            .ok_or_else(|| BookError::new(format!("Account {} not defined!", ref1)))?
            .add_transaction(row.transaction_date, row.amount, BankTransactionReferences::new_from_single(row.reference.as_str()))?;
    }

    Ok(())
}