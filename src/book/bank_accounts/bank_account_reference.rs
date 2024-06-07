#[derive(Debug, Clone, Hash, PartialEq, Eq, serde::Deserialize)]
pub struct SwedishAccountNumber {
    pub number: i64
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, serde::Deserialize)]
pub struct NamedAccount {
    pub name: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, serde::Deserialize)]
pub enum BankAccountReference {
    #[serde(rename="swedish_account_number")]
    SwedishAccountNumber(SwedishAccountNumber),
    #[serde(rename="named_account")]
    NamedAccount(NamedAccount)
}

impl std::fmt::Display for BankAccountReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SwedishAccountNumber(e) => write!(f, "Swedish Account Number = {}", e.number),
            Self::NamedAccount(e) => write!(f, "Named account = {}", e.name)
        }
    }
}