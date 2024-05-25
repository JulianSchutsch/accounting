#[derive(Debug, serde::Deserialize)]
pub enum Method {
    #[serde(rename="normal")]
    Normal,
    #[serde(rename="reverse-charge")]
    ReverseCharge
}

#[derive(Debug, serde::Deserialize)]
pub enum Category {
    #[serde(rename="services")]
    Services,
}

#[derive(serde::Deserialize)]
pub struct Income {
  pub id: String,
  #[serde(with="time::serde::iso8601")]
  pub date: time::OffsetDateTime,
  pub country: crate::book::country::Country,
  #[serde(rename="customer-vat")]
  pub customer_vat: String,
  pub currency: String,
  pub method: Method,
  pub category: Category,
  pub amount: i64,
  pub moms: i64,
  pub description: String,
}

use runtime_format::FormatKeyError;

impl runtime_format::FormatKey for Income {
    fn fmt(&self, key: &str, f: &mut std::fmt::Formatter<'_>) -> Result<(), FormatKeyError> {
        match key {
            "id" => write!(f, "{}", self.id).map_err(FormatKeyError::Fmt),
            "method" => write!(f, "{:?}", self.method).map_err(FormatKeyError::Fmt),
            _ => Err(runtime_format::FormatKeyError::UnknownKey),
        }
    }
}
