//! Logical Lines of Code (LLOC) primitive.

use std::iter::Sum;
use std::ops::Add;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{DomainPrimitive, Validated};

/// Logical Lines of Code - a non-negative count of logical statements.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Lloc(f64);

#[derive(Debug, Error, Clone, PartialEq)]
pub enum LlocValidationError {
    #[error("LLOC cannot be negative: {0}")]
    Negative(f64),

    #[error("LLOC cannot be NaN")]
    NaN,
}

impl Lloc {
    pub fn new(value: f64) -> Result<Self, LlocValidationError> {
        if value.is_nan() {
            return Err(LlocValidationError::NaN);
        }
        if value < 0.0 {
            return Err(LlocValidationError::Negative(value));
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

impl Validated for Lloc {
    type ValidationError = LlocValidationError;
}

impl DomainPrimitive for Lloc {}

impl Add for Lloc {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sum for Lloc {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, x| acc + x)
    }
}

impl TryFrom<f64> for Lloc {
    type Error = LlocValidationError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Lloc> for f64 {
    fn from(lloc: Lloc) -> f64 {
        lloc.0
    }
}
