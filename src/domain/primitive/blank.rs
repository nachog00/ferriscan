//! Blank Lines primitive.

use std::iter::Sum;
use std::ops::Add;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{DomainPrimitive, Validated};

/// Blank Lines - a count of empty lines in code.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Blank(f64);

#[derive(Debug, Error, Clone, PartialEq)]
pub enum BlankValidationError {
    #[error("blank lines cannot be negative: {0}")]
    Negative(f64),

    #[error("blank lines cannot be NaN")]
    NaN,
}

impl Blank {
    pub fn new(value: f64) -> Result<Self, BlankValidationError> {
        if value.is_nan() {
            return Err(BlankValidationError::NaN);
        }
        if value < 0.0 {
            return Err(BlankValidationError::Negative(value));
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

impl Validated for Blank {
    type ValidationError = BlankValidationError;
}

impl DomainPrimitive for Blank {}

impl Add for Blank {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sum for Blank {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, x| acc + x)
    }
}

impl TryFrom<f64> for Blank {
    type Error = BlankValidationError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Blank> for f64 {
    fn from(blank: Blank) -> f64 {
        blank.0
    }
}
