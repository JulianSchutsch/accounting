#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub struct MomsPercent(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, serde::Deserialize)]
pub struct Amount(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct MomsClassedAmount {
    pub moms_percent: MomsPercent,
    pub amount: Amount,
    pub moms: Amount
}

impl MomsClassedAmount {
    pub fn verify(&self) -> bool {
        let moms_error = (self.moms_percent.0 as f64)/100.0 * self.amount.0 - self.moms.0;
        return moms_error<=0.02;
    }
}

impl<'de> serde::Deserialize<'de> for MomsClassedAmount {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<MomsClassedAmount, D::Error> {
        #[derive(serde::Deserialize)]
        struct Tuple(pub i32, pub f64, pub f64);
        let data = Tuple::deserialize(deserializer)?;
        Ok(MomsClassedAmount{ moms_percent: MomsPercent(data.0), amount: Amount(data.1), moms: Amount(data.2) })
    }
}

impl std::ops::Add for Amount {
    type Output = Amount;
    fn add(self, rhs: Self) -> Self::Output {
        return Self(self.0+rhs.0);
    }
}

impl std::ops::Sub for Amount {
    type Output = Amount;
    fn sub(self, rhs: Self) -> Self::Output {
        return Self(self.0-rhs.0);
    }
}

impl std::fmt::Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
