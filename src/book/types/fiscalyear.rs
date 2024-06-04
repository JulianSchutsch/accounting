use super::date::Date;

#[derive(serde::Deserialize)]
pub struct FiscalYear {
    pub begin: Date,
    pub end: Date
}