//! Halstead complexity metrics primitives.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{DomainPrimitive, Validated};

// ============================================================================
// Halstead Difficulty
// ============================================================================

/// Halstead Difficulty - measures how difficult code is to write/understand.
/// Higher values indicate more difficult code.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HalsteadDifficulty(f64);

#[derive(Debug, Error, Clone, PartialEq)]
pub enum HalsteadDifficultyValidationError {
    #[error("Halstead difficulty cannot be negative: {0}")]
    Negative(f64),

    #[error("Halstead difficulty cannot be NaN")]
    NaN,
}

impl HalsteadDifficulty {
    pub fn new(value: f64) -> Result<Self, HalsteadDifficultyValidationError> {
        if value.is_nan() {
            return Err(HalsteadDifficultyValidationError::NaN);
        }
        if value < 0.0 {
            return Err(HalsteadDifficultyValidationError::Negative(value));
        }
        Ok(Self(value))
    }

    pub const fn zero() -> Self {
        Self(0.0)
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

impl Validated for HalsteadDifficulty {
    type ValidationError = HalsteadDifficultyValidationError;
}

impl DomainPrimitive for HalsteadDifficulty {}

impl TryFrom<f64> for HalsteadDifficulty {
    type Error = HalsteadDifficultyValidationError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<HalsteadDifficulty> for f64 {
    fn from(hd: HalsteadDifficulty) -> f64 {
        hd.0
    }
}

// ============================================================================
// Halstead Effort
// ============================================================================

/// Halstead Effort - measures mental effort required to develop/maintain code.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HalsteadEffort(f64);

#[derive(Debug, Error, Clone, PartialEq)]
pub enum HalsteadEffortValidationError {
    #[error("Halstead effort cannot be negative: {0}")]
    Negative(f64),

    #[error("Halstead effort cannot be NaN")]
    NaN,
}

impl HalsteadEffort {
    pub fn new(value: f64) -> Result<Self, HalsteadEffortValidationError> {
        if value.is_nan() {
            return Err(HalsteadEffortValidationError::NaN);
        }
        if value < 0.0 {
            return Err(HalsteadEffortValidationError::Negative(value));
        }
        Ok(Self(value))
    }

    pub const fn zero() -> Self {
        Self(0.0)
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

impl Validated for HalsteadEffort {
    type ValidationError = HalsteadEffortValidationError;
}

impl DomainPrimitive for HalsteadEffort {}

impl TryFrom<f64> for HalsteadEffort {
    type Error = HalsteadEffortValidationError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<HalsteadEffort> for f64 {
    fn from(he: HalsteadEffort) -> f64 {
        he.0
    }
}

// ============================================================================
// Halstead Bugs
// ============================================================================

/// Halstead Bugs - estimated number of bugs in code.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HalsteadBugs(f64);

#[derive(Debug, Error, Clone, PartialEq)]
pub enum HalsteadBugsValidationError {
    #[error("Halstead bugs cannot be negative: {0}")]
    Negative(f64),

    #[error("Halstead bugs cannot be NaN")]
    NaN,
}

impl HalsteadBugs {
    pub fn new(value: f64) -> Result<Self, HalsteadBugsValidationError> {
        if value.is_nan() {
            return Err(HalsteadBugsValidationError::NaN);
        }
        if value < 0.0 {
            return Err(HalsteadBugsValidationError::Negative(value));
        }
        Ok(Self(value))
    }

    pub const fn zero() -> Self {
        Self(0.0)
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

impl Validated for HalsteadBugs {
    type ValidationError = HalsteadBugsValidationError;
}

impl DomainPrimitive for HalsteadBugs {}

impl TryFrom<f64> for HalsteadBugs {
    type Error = HalsteadBugsValidationError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<HalsteadBugs> for f64 {
    fn from(hb: HalsteadBugs) -> f64 {
        hb.0
    }
}
