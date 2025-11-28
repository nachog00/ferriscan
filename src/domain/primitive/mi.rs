//! Maintainability Index (MI) primitive.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{DomainPrimitive, Validated};

/// Maintainability Index - a composite metric measuring code maintainability.
/// Higher values indicate more maintainable code.
/// Typically ranges from 0-171, but we only enforce >= 0.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MaintainabilityIndex(f64);

#[derive(Debug, Error, Clone, PartialEq)]
pub enum MiValidationError {
    #[error("maintainability index cannot be negative: {0}")]
    Negative(f64),

    #[error("maintainability index cannot be NaN")]
    NaN,
}

impl MaintainabilityIndex {
    pub fn new(value: f64) -> Result<Self, MiValidationError> {
        if value.is_nan() {
            return Err(MiValidationError::NaN);
        }
        if value < 0.0 {
            return Err(MiValidationError::Negative(value));
        }
        Ok(Self(value))
    }

    pub const fn zero() -> Self {
        Self(0.0)
    }

    pub fn value(self) -> f64 {
        self.0
    }

    /// Check if the MI indicates highly maintainable code (>= 80).
    pub fn is_highly_maintainable(self) -> bool {
        self.0 >= 80.0
    }

    /// Check if the MI indicates moderately maintainable code (20-80).
    pub fn is_moderately_maintainable(self) -> bool {
        self.0 >= 20.0 && self.0 < 80.0
    }

    /// Check if the MI indicates difficult to maintain code (< 20).
    pub fn is_difficult_to_maintain(self) -> bool {
        self.0 < 20.0
    }
}

impl Validated for MaintainabilityIndex {
    type ValidationError = MiValidationError;
}

impl DomainPrimitive for MaintainabilityIndex {}

impl TryFrom<f64> for MaintainabilityIndex {
    type Error = MiValidationError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<MaintainabilityIndex> for f64 {
    fn from(mi: MaintainabilityIndex) -> f64 {
        mi.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_mi() {
        assert!(MaintainabilityIndex::new(0.0).is_ok());
        assert!(MaintainabilityIndex::new(100.0).is_ok());
        assert!(MaintainabilityIndex::new(171.0).is_ok());
    }

    #[test]
    fn negative_rejected() {
        assert!(matches!(
            MaintainabilityIndex::new(-1.0),
            Err(MiValidationError::Negative(_))
        ));
    }

    #[test]
    fn nan_rejected() {
        assert!(matches!(
            MaintainabilityIndex::new(f64::NAN),
            Err(MiValidationError::NaN)
        ));
    }

    #[test]
    fn maintainability_categories() {
        let high = MaintainabilityIndex::new(85.0).unwrap();
        assert!(high.is_highly_maintainable());
        assert!(!high.is_moderately_maintainable());
        assert!(!high.is_difficult_to_maintain());

        let moderate = MaintainabilityIndex::new(50.0).unwrap();
        assert!(!moderate.is_highly_maintainable());
        assert!(moderate.is_moderately_maintainable());
        assert!(!moderate.is_difficult_to_maintain());

        let difficult = MaintainabilityIndex::new(10.0).unwrap();
        assert!(!difficult.is_highly_maintainable());
        assert!(!difficult.is_moderately_maintainable());
        assert!(difficult.is_difficult_to_maintain());
    }
}
