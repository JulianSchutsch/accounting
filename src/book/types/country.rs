#[derive(Debug, serde::Deserialize)]
pub enum Country {
    #[serde(rename="sweden")]
    Sweden,
    #[serde(rename="switzerland")]
    Switzerland,
}
