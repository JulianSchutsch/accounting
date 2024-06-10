#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
pub enum Category {
    #[serde(rename="software-license")]
    SoftwareLicense,
    #[serde(rename="media-advertisement")]
    MediaAdvertisement,
    #[serde(rename="services")]
    Services,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::SoftwareLicense => write!(f, "Software license"),
            Category::MediaAdvertisement => write!(f, "Media advertisment"),
            Category::Services => write!(f, "Services")
        }
    }
}