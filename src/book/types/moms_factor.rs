use crate::book::*;

#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub struct MomsFactor(pub f64);

impl std::ops::Mul<Amount> for MomsFactor {
    type Output = Amount;
    fn mul(self, rhs: Amount) -> Self::Output {
        Amount( self.0 * rhs.0 )
    }
}