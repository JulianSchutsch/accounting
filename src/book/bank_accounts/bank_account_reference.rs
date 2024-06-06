#[derive(Clone, Hash, PartialEq, Eq, serde::Deserialize)]
pub struct SwedishAccountNumber {
    pub number: i64
}

#[derive(Clone, Hash, PartialEq, Eq, serde::Deserialize)]
pub struct Name {
    pub name: String,
}

#[derive(Clone, Hash, PartialEq, Eq, serde::Deserialize)]
pub enum BankAccountReference {
    #[serde(rename="swedish_account_number")]
    SwedishAccountNumber(SwedishAccountNumber),
    #[serde(rename="name")]
    Name(Name)
}