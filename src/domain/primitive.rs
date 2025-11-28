//! The DomainPrimitive trait and foundational types for strongly-typed metrics.
//!
//! Each domain primitive:
//! - Implements `Validated` with a `ValidationError` for construction failures
//! - Implements `DomainPrimitive` as a marker for the family
//! - Optionally implements `Operative` if it has fallible operations
//!
//! By the time data enters the domain layer, it's valid by construction.

pub mod cognitive;
pub mod cyclomatic;
pub mod lloc;
pub mod mi;
pub mod sloc;

use std::error::Error;

/// A type that validates on construction.
///
/// The `ValidationError` describes what invariants were violated.
/// Validation happens at the extraction boundary; the domain layer
/// only sees valid instances.
pub trait Validated: Sized {
    /// Error type for construction/validation failures.
    type ValidationError: Error + Send + Sync + 'static;
}

/// A type that has fallible operations.
///
/// The `OpError` describes operation failures on valid instances
/// (e.g., division by zero, underflow).
pub trait Operative: Sized {
    /// Error type for operation failures.
    type OpError: Error + Send + Sync + 'static;
}

/// Marker trait for all domain primitives.
///
/// All domain primitives are `Validated`. Use this trait for blanket
/// implementations over the primitive family. Types that also have
/// fallible operations implement `Operative` separately.
pub trait DomainPrimitive: Validated {}
