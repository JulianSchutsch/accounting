use crate::book::*;

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub struct Cost {
    pub cost: Amount
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub struct Exchanged {
    pub exchanged: Amount
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(rename_all="snake_case")]
pub enum Payment {
    Exact,
    ExactMedCost(Cost),
    ExactExchanged(Exchanged),
    LeaderCredit
}