use core::fmt;
use std::fmt::Formatter;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, serde::Deserialize)]
pub enum Currency {
    SEK,
    EUR
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::SEK => write!(f, "SEK"),
            Currency::EUR => write!(f, "EUR"),
        }
    }
}