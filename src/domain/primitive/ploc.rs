//! Physical Lines of Code (PLOC) primitive.

use std::iter::Sum;
use std::ops::Add;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{DomainPrimitive, Validated};

/// Physical Lines of Code - a count of all lines including blanks and comments.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Ploc(f64);

#[derive(Debug, Error, Clone, PartialEq)]
pub enum PlocValidationError {
    #[error("PLOC cannot be negative: {0}")]
    Negative(f64),

    #[error("PLOC cannot be NaN")]
    NaN,
}

impl Ploc {
    pub fn new(value: f64) -> Result<Self, PlocValidationError> {
        if value.is_nan() {
            return Err(PlocValidationError::NaN);
        }
        if value < 0.0 {
            return Err(PlocValidationError::Negative(value));
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

impl Validated for Ploc {
    type ValidationError = PlocValidationError;
}

impl DomainPrimitive for Ploc {}

impl Add for Ploc {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sum for Ploc {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, x| acc + x)
    }
}

impl TryFrom<f64> for Ploc {
    type Error = PlocValidationError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Ploc> for f64 {
    fn from(ploc: Ploc) -> f64 {
        ploc.0
    }
}
