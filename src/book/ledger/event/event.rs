use crate::book::*;
use crate::book::ledger::event::end_fiscal_year::EndFiscalYear;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all="snake_case")]
pub enum Event {
    Income(Income),
    Invoice(Invoice),
    Salary(Salary),
    TaxPayment(TaxPayment),
    Interest(Interest),
    Fine(Fine),
    VerifyMoms(VerifyMoms),
    Shares(Shares),
    Transaction(Transaction),
    BankCost(BankCost),
    Exchange(Exchange),
    EndFiscalYear(EndFiscalYear)
}

impl Event {
    pub fn date(&self) -> Date {
        match self {
            Event::Income(e) => e.date,
            Event::Invoice(e) => e.date,
            Event::Salary(e) => e.date,
            Event::TaxPayment(e) => e.date,
            Event::Interest(e) => e.date,
            Event::Fine(e) => e.date,
            Event::VerifyMoms(e) => e.date,
            Event::Shares(e) => e.date,
            Event::Transaction(e) => e.date,
            Event::BankCost(e) => e.date,
            Event::Exchange(e) => e.date,
            Event::EndFiscalYear(e) => e.date
        }
    }

    pub fn id(&self) -> &String {
        match self {
            Event::Income(e) => &e.id,
            Event::Invoice(e) => &e.id,
            Event::Salary(e) => &e.id,
            Event::TaxPayment(e) => &e.id,
            Event::Interest(e) => &e.id,
            Event::Fine(e) => &e.id,
            Event::VerifyMoms(e) => &e.id,
            Event::Shares(e) => &e.id,
            Event::Transaction(e) => &e.id,
            Event::BankCost(e) => &e.id,
            Event::Exchange(e) => &e.id,
            Event::EndFiscalYear(e) => &e.id
        }
    }
}
