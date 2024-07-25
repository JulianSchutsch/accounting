#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
#[serde(rename_all="snake_case")]
pub enum Category {
    SoftwareLicense,
    MediaAdvertisement,
    MomsFreeCost,
    Services,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::SoftwareLicense => write!(f, "Software license"),
            Category::MediaAdvertisement => write!(f, "Media advertisment"),
            Category::MomsFreeCost => write!(f, "Moms free cost"),
            Category::Services => write!(f, "Services")
        }
    }
}