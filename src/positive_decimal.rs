use derive_more::{Display, Into};
use rust_decimal::Decimal;
use serde::Deserialize;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Display, Eq, Hash, Into, Ord, PartialEq, PartialOrd)]
pub struct PositiveDecimal(Decimal);

#[derive(Debug, Error)]
pub enum PositiveDecimalError {
    #[error("negative amount")]
    NegativeAmount,
    #[error("zero amount")]
    ZeroAmount,
}

impl PositiveDecimal {
    pub fn new(value: Decimal) -> Result<Self, PositiveDecimalError> {
        if value.is_sign_negative() {
            return Err(PositiveDecimalError::NegativeAmount);
        }

        if value.is_zero() {
            return Err(PositiveDecimalError::ZeroAmount);
        }

        Ok(PositiveDecimal(value))
    }
}

impl<'de> Deserialize<'de> for PositiveDecimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <Decimal as Deserialize>::deserialize(deserializer)?;
        PositiveDecimal::new(value).map_err(serde::de::Error::custom)
    }
}

impl TryFrom<Decimal> for PositiveDecimal {
    type Error = PositiveDecimalError;

    fn try_from(value: Decimal) -> Result<Self, Self::Error> {
        PositiveDecimal::new(value)
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn positive_decimal_success() {
        PositiveDecimal::new(dec!(5.0)).unwrap();
    }

    #[test]
    fn positive_decimal_failure() {
        assert!(PositiveDecimal::new(dec!(-5.0)).is_err());
        assert!(PositiveDecimal::new(dec!(0.0)).is_err());
    }
}
