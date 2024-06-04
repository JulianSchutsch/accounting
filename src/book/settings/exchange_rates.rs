mod riksbank;

pub use riksbank::Riksbank;

#[derive(serde::Deserialize)]
pub enum ExchangeRate {
    #[serde(rename="riksbank")]
    Riksbank(Riksbank)
}
