#[derive(Debug, serde::Deserialize)]
pub enum PaymentKind {
    #[serde(rename="bank-wire")]
    BankWire,
    #[serde(rename="swish")]
    Swish
}
