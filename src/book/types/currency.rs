#[derive(Debug, Hash, Eq, PartialEq, Clone, enum_map::Enum, Copy, PartialOrd, Ord, serde::Deserialize)]
pub enum Currency {
    SEK,
    EUR
}

pub const ALL_CURRENCIES: [Currency;2] = [Currency::SEK, Currency::EUR];

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::SEK => write!(f, "SEK"),
            Currency::EUR => write!(f, "EUR"),
        }
    }
}