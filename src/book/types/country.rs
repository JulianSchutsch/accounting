#[derive(PartialEq, Debug, Clone, Copy, serde::Deserialize)]
pub enum Country {
    #[serde(rename="sweden")]
    Sweden,
    #[serde(rename="switzerland")]
    Switzerland,
}

impl Country {
    pub fn is_eu(&self) -> bool {
        match self {
            Country::Sweden => true,
            _ => false,
        }
    }
}