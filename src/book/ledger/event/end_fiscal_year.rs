use crate::book::*;

#[derive(Debug, serde::Deserialize)]
pub struct EndFiscalYear {
    pub id: String,
    pub fiscal_year: Period
}
