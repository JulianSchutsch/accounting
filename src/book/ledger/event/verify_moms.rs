use crate::book::*;

#[derive(Debug, serde::Deserialize)]
pub struct VerifyMoms {
    pub id: String,
    pub date: Date,
}
