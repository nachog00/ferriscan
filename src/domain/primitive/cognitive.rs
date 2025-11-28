//! Cognitive Complexity primitive.

use std::iter::Sum;
use std::ops::Add;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{DomainPrimitive, Validated};

/// Cognitive Complexity - measures how difficult code is to understand.
/// Unlike cyclomatic complexity, can be 0 for trivial functions.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CognitiveComplexity(f64);

#[derive(Debug, Error, Clone, PartialEq)]
pub enum CognitiveValidationError {
    #[error("cognitive complexity cannot be negative: {0}")]
    Negative(f64),

    #[error("cognitive complexity cannot be NaN")]
    NaN,
}

impl CognitiveComplexity {
    pub fn new(value: f64) -> Result<Self, CognitiveValidationError> {
        if value.is_nan() {
            return Err(CognitiveValidationError::NaN);
        }
        if value < 0.0 {
            return Err(CognitiveValidationError::Negative(value));
        }
        Ok(Self(value))
    }

    pub const fn zero() -> Self {
        Self(0.0)
    }

    pub fn value(self) -> f64 {
        self.0
    }

    pub fn is_zero(self) -> bool {
        self.0 == 0.0
    }
}

impl Validated for CognitiveComplexity {
    type ValidationError = CognitiveValidationError;
}

impl DomainPrimitive for CognitiveComplexity {}

impl Add for CognitiveComplexity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sum for CognitiveComplexity {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, x| acc + x)
    }
}

impl TryFrom<f64> for CognitiveComplexity {
    type Error = CognitiveValidationError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CognitiveComplexity> for f64 {
    fn from(cc: CognitiveComplexity) -> f64 {
        cc.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_cognitive() {
        assert!(CognitiveComplexity::new(0.0).is_ok());
        assert!(CognitiveComplexity::new(10.0).is_ok());
        assert!(CognitiveComplexity::new(0.5).is_ok());
    }

    #[test]
    fn negative_rejected() {
        assert!(matches!(
            CognitiveComplexity::new(-1.0),
            Err(CognitiveValidationError::Negative(_))
        ));
    }

    #[test]
    fn nan_rejected() {
        assert!(matches!(
            CognitiveComplexity::new(f64::NAN),
            Err(CognitiveValidationError::NaN)
        ));
    }

    #[test]
    fn addition() {
        let a = CognitiveComplexity::new(5.0).unwrap();
        let b = CognitiveComplexity::new(3.0).unwrap();
        assert_eq!((a + b).value(), 8.0);
    }

    #[test]
    fn sum_iter() {
        let values = vec![
            CognitiveComplexity::new(1.0).unwrap(),
            CognitiveComplexity::new(2.0).unwrap(),
            CognitiveComplexity::new(3.0).unwrap(),
        ];
        let total: CognitiveComplexity = values.into_iter().sum();
        assert_eq!(total.value(), 6.0);
    }
}
