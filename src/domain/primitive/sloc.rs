//! Source Lines of Code (SLOC) primitive.

use std::iter::Sum;
use std::ops::Add;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{DomainPrimitive, Operative, Validated};

/// Source Lines of Code - a non-negative count of source lines.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Sloc(f64);

#[derive(Debug, Error, Clone, PartialEq)]
pub enum SlocValidationError {
    #[error("SLOC cannot be negative: {0}")]
    Negative(f64),

    #[error("SLOC cannot be NaN")]
    NaN,
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum SlocOpError {
    #[error("cannot divide by zero SLOC")]
    DivisionByZero,
}

impl Sloc {
    /// Create a new SLOC value, validating that it's non-negative and not NaN.
    pub fn new(value: f64) -> Result<Self, SlocValidationError> {
        if value.is_nan() {
            return Err(SlocValidationError::NaN);
        }
        if value < 0.0 {
            return Err(SlocValidationError::Negative(value));
        }
        Ok(Self(value))
    }

    /// Create a SLOC of zero.
    pub const fn zero() -> Self {
        Self(0.0)
    }

    /// Get the raw value.
    pub fn value(self) -> f64 {
        self.0
    }

    /// Check if this is zero.
    pub fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    /// Divide a value by this SLOC, returning an error if zero.
    pub fn divide_into(self, numerator: f64) -> Result<f64, SlocOpError> {
        if self.is_zero() {
            return Err(SlocOpError::DivisionByZero);
        }
        Ok(numerator / self.0)
    }
}

impl Validated for Sloc {
    type ValidationError = SlocValidationError;
}

impl Operative for Sloc {
    type OpError = SlocOpError;
}

impl DomainPrimitive for Sloc {}

impl Add for Sloc {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // Addition of two valid Sloc values is always valid
        Self(self.0 + rhs.0)
    }
}

impl Sum for Sloc {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, x| acc + x)
    }
}

impl TryFrom<f64> for Sloc {
    type Error = SlocValidationError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Sloc> for f64 {
    fn from(sloc: Sloc) -> f64 {
        sloc.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_sloc() {
        assert!(Sloc::new(0.0).is_ok());
        assert!(Sloc::new(100.0).is_ok());
        assert!(Sloc::new(0.5).is_ok());
    }

    #[test]
    fn negative_sloc_rejected() {
        assert!(matches!(
            Sloc::new(-1.0),
            Err(SlocValidationError::Negative(_))
        ));
    }

    #[test]
    fn nan_sloc_rejected() {
        assert!(matches!(Sloc::new(f64::NAN), Err(SlocValidationError::NaN)));
    }

    #[test]
    fn sloc_addition() {
        let a = Sloc::new(10.0).unwrap();
        let b = Sloc::new(20.0).unwrap();
        assert_eq!((a + b).value(), 30.0);
    }

    #[test]
    fn sloc_sum() {
        let values = vec![
            Sloc::new(10.0).unwrap(),
            Sloc::new(20.0).unwrap(),
            Sloc::new(30.0).unwrap(),
        ];
        let total: Sloc = values.into_iter().sum();
        assert_eq!(total.value(), 60.0);
    }

    #[test]
    fn division_by_zero() {
        let zero = Sloc::zero();
        assert!(matches!(
            zero.divide_into(100.0),
            Err(SlocOpError::DivisionByZero)
        ));
    }

    #[test]
    fn division_success() {
        let sloc = Sloc::new(4.0).unwrap();
        assert_eq!(sloc.divide_into(100.0).unwrap(), 25.0);
    }
}
