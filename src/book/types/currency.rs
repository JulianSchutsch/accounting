use crate::book::*;

#[derive(Debug, Hash, Eq, PartialEq, Clone, enum_map::Enum, Copy, serde::Deserialize)]
pub enum Currency {
    SEK,
    EUR
}

impl Currency {
    pub fn from_str(s:&str) -> BookResult<Self> {
        match s {
            "SEK" => Ok(Currency::SEK),
            "EUR" => Ok(Currency::EUR),
            _ => Err(BookError::new(format!("Invalid currency string {}", s)))
        }
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::SEK => write!(f, "SEK"),
            Currency::EUR => write!(f, "EUR"),
        }
    }
}