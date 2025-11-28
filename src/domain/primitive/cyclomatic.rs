//! Cyclomatic Complexity primitive.

use std::iter::Sum;
use std::ops::Add;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{DomainPrimitive, Validated};

/// Cyclomatic Complexity - measures independent paths through code.
/// Minimum value is 1 (a function with no branches).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CyclomaticComplexity(f64);

#[derive(Debug, Error, Clone, PartialEq)]
pub enum CyclomaticValidationError {
    #[error("cyclomatic complexity must be >= 1, got {0}")]
    BelowMinimum(f64),

    #[error("cyclomatic complexity cannot be NaN")]
    NaN,
}

impl CyclomaticComplexity {
    pub fn new(value: f64) -> Result<Self, CyclomaticValidationError> {
        if value.is_nan() {
            return Err(CyclomaticValidationError::NaN);
        }
        if value < 1.0 {
            return Err(CyclomaticValidationError::BelowMinimum(value));
        }
        Ok(Self(value))
    }

    /// The minimum cyclomatic complexity (a single path through code).
    pub const fn one() -> Self {
        Self(1.0)
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

impl Validated for CyclomaticComplexity {
    type ValidationError = CyclomaticValidationError;
}

impl DomainPrimitive for CyclomaticComplexity {}

impl Add for CyclomaticComplexity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sum for CyclomaticComplexity {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::one(), |acc, x| Self(acc.0 + x.0 - 1.0))
    }
}

impl TryFrom<f64> for CyclomaticComplexity {
    type Error = CyclomaticValidationError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CyclomaticComplexity> for f64 {
    fn from(cc: CyclomaticComplexity) -> f64 {
        cc.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_cyclomatic() {
        assert!(CyclomaticComplexity::new(1.0).is_ok());
        assert!(CyclomaticComplexity::new(10.0).is_ok());
        assert!(CyclomaticComplexity::new(1.5).is_ok());
    }

    #[test]
    fn below_minimum_rejected() {
        assert!(matches!(
            CyclomaticComplexity::new(0.0),
            Err(CyclomaticValidationError::BelowMinimum(_))
        ));
        assert!(matches!(
            CyclomaticComplexity::new(0.5),
            Err(CyclomaticValidationError::BelowMinimum(_))
        ));
        assert!(matches!(
            CyclomaticComplexity::new(-1.0),
            Err(CyclomaticValidationError::BelowMinimum(_))
        ));
    }

    #[test]
    fn nan_rejected() {
        assert!(matches!(
            CyclomaticComplexity::new(f64::NAN),
            Err(CyclomaticValidationError::NaN)
        ));
    }

    #[test]
    fn addition() {
        let a = CyclomaticComplexity::new(5.0).unwrap();
        let b = CyclomaticComplexity::new(3.0).unwrap();
        assert_eq!((a + b).value(), 8.0);
    }
}
