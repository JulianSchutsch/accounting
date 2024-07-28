use crate::book::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CheckPeriod {
    pub id: String,
    pub period: Period,
}
