//! Integer count primitives (Nargs, Nexits, LineNumber).

use std::iter::Sum;
use std::ops::Add;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{DomainPrimitive, Validated};

// ============================================================================
// Nargs - Number of function arguments
// ============================================================================

/// Number of arguments in a function signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Nargs(usize);

#[derive(Debug, Error, Clone, PartialEq)]
pub enum NargsValidationError {
    // usize cannot be negative, so no validation errors possible
    // but we keep the error type for trait consistency
    #[error("nargs validation error (unreachable)")]
    Unreachable,
}

impl Nargs {
    pub fn new(value: usize) -> Result<Self, NargsValidationError> {
        Ok(Self(value))
    }

    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> usize {
        self.0
    }

    pub fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl Validated for Nargs {
    type ValidationError = NargsValidationError;
}

impl DomainPrimitive for Nargs {}

impl Add for Nargs {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sum for Nargs {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, x| acc + x)
    }
}

impl From<usize> for Nargs {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Nargs> for usize {
    fn from(nargs: Nargs) -> usize {
        nargs.0
    }
}

// ============================================================================
// Nexits - Number of exit points
// ============================================================================

/// Number of exit points in a function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Nexits(usize);

#[derive(Debug, Error, Clone, PartialEq)]
pub enum NexitsValidationError {
    #[error("nexits validation error (unreachable)")]
    Unreachable,
}

impl Nexits {
    pub fn new(value: usize) -> Result<Self, NexitsValidationError> {
        Ok(Self(value))
    }

    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn value(self) -> usize {
        self.0
    }

    pub fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl Validated for Nexits {
    type ValidationError = NexitsValidationError;
}

impl DomainPrimitive for Nexits {}

impl Add for Nexits {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sum for Nexits {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, x| acc + x)
    }
}

impl From<usize> for Nexits {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Nexits> for usize {
    fn from(nexits: Nexits) -> usize {
        nexits.0
    }
}

// ============================================================================
// LineNumber - Source line number (1-indexed)
// ============================================================================

/// A line number in source code (1-indexed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LineNumber(usize);

#[derive(Debug, Error, Clone, PartialEq)]
pub enum LineNumberValidationError {
    #[error("line number must be >= 1, got {0}")]
    Zero(usize),
}

impl LineNumber {
    pub fn new(value: usize) -> Result<Self, LineNumberValidationError> {
        if value == 0 {
            return Err(LineNumberValidationError::Zero(value));
        }
        Ok(Self(value))
    }

    pub const fn one() -> Self {
        Self(1)
    }

    pub fn value(self) -> usize {
        self.0
    }
}

impl Validated for LineNumber {
    type ValidationError = LineNumberValidationError;
}

impl DomainPrimitive for LineNumber {}

impl TryFrom<usize> for LineNumber {
    type Error = LineNumberValidationError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<LineNumber> for usize {
    fn from(ln: LineNumber) -> usize {
        ln.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nargs_valid() {
        assert!(Nargs::new(0).is_ok());
        assert!(Nargs::new(10).is_ok());
        assert_eq!(Nargs::new(5).unwrap().value(), 5);
    }

    #[test]
    fn nargs_addition() {
        let a = Nargs::new(3).unwrap();
        let b = Nargs::new(2).unwrap();
        assert_eq!((a + b).value(), 5);
    }

    #[test]
    fn nexits_valid() {
        assert!(Nexits::new(0).is_ok());
        assert!(Nexits::new(5).is_ok());
    }

    #[test]
    fn line_number_valid() {
        assert!(LineNumber::new(1).is_ok());
        assert!(LineNumber::new(100).is_ok());
    }

    #[test]
    fn line_number_zero_rejected() {
        assert!(matches!(
            LineNumber::new(0),
            Err(LineNumberValidationError::Zero(_))
        ));
    }
}
