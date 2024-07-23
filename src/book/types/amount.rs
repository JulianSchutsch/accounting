pub const EPSILON: Amount = Amount(0.0001);

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, serde::Deserialize)]
pub struct Amount(pub f64);

pub fn almost_equal(a: Amount, b: Amount) -> bool {
    (a-b).abs()<EPSILON
}

impl Amount {
    pub fn abs(&self) -> Self {
        return Amount(self.0.abs());
    }
}

impl std::ops::Neg for Amount {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self( -self.0 )
    }
}

impl std::ops::AddAssign for Amount {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
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
        write!(f, "{:.2}", self.0)
    }
}
