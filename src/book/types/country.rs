#[derive(PartialEq, Debug, Clone, Copy, serde::Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Country {
    Sweden,
    Switzerland,
    Germany,
    Netherlands
}

impl Country {
    pub fn is_eu(&self) -> bool {
        match self {
            Country::Sweden |
            Country::Netherlands |
            Country::Germany => true,
            _ => false,
        }
    }
}