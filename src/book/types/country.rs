#[derive(PartialEq, Debug, Clone, Copy, serde::Deserialize)]
#[serde(rename_all="snake_case")]
pub enum Country {
    Sweden,
    Switzerland,
    Germany,
    Netherlands,
    CzechRepublic
}

impl Country {
    pub fn is_eu(&self) -> bool {
        match self {
            Country::Sweden |
            Country::Netherlands |
            Country::CzechRepublic |
            Country::Germany => true,
            _ => false,
        }
    }
}