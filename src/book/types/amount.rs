#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub struct MomsPerc(pub i32);

#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub struct Amount(pub f64);

#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub struct BookAmount(pub f64);

#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub struct Moms(pub f64);

#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub struct MomsClassedAmount(pub MomsPerc, pub Amount, pub Moms);