use crate::book::*;

#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(rename_all="snake_case")]
pub enum Payment {
    Exact,
    LeaderCredit
}