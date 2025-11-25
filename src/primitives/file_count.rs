use derive_more::{Add, AddAssign, Display, From, Into};
use serde::{Deserialize, Serialize};
use std::iter::Sum;
use std::ops::{Div, Sub};

/// A count of files, guaranteed non-negative.
///
/// Subtraction produces a signed `i64` diff, avoiding manual casting
/// when comparing counts.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord,
    Add, AddAssign, Display, From, Into,
    Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct FileCount(u64);

impl FileCount {
    pub const ZERO: Self = Self(0);

    pub fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub fn as_f64(self) -> f64 {
        self.0 as f64
    }
}

impl From<usize> for FileCount {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

impl Sub for FileCount {
    type Output = i64;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 as i64 - rhs.0 as i64
    }
}

impl Sum for FileCount {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, x| acc + x)
    }
}

/// Division by FileCount returns 0.0 when dividing by zero.
impl Div<FileCount> for f64 {
    type Output = f64;

    fn div(self, rhs: FileCount) -> f64 {
        if rhs.is_zero() {
            0.0
        } else {
            self / rhs.as_f64()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_zero() {
        assert_eq!(FileCount::default(), FileCount(0));
    }

    #[test]
    fn subtraction_produces_signed_diff() {
        let a = FileCount(10);
        let b = FileCount(15);

        assert_eq!(a - b, -5);
        assert_eq!(b - a, 5);
    }

    #[test]
    fn addition_works() {
        let a = FileCount(10);
        let b = FileCount(5);

        assert_eq!(a + b, FileCount(15));
    }

    #[test]
    fn from_and_into_u64() {
        let count: FileCount = 42u64.into();
        let n: u64 = count.into();
        assert_eq!(n, 42);
    }

    #[test]
    fn division_by_filecount() {
        let count = FileCount(4);
        assert_eq!(100.0 / count, 25.0);
    }

    #[test]
    fn division_by_zero_returns_zero() {
        let count = FileCount::ZERO;
        assert_eq!(100.0 / count, 0.0);
    }
}
