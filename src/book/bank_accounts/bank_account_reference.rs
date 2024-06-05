#[derive(Clone, Copy, Hash, PartialEq, Eq, serde::Deserialize)]
pub struct SwedishAccountNumber {
    pub number: i64
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, serde::Deserialize)]
pub enum BankAccountReference {
    #[serde(rename="swedish_account_number")]
    Swedish_Account_Number(SwedishAccountNumber)
}