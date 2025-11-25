pub mod empty;
pub mod types_only;

/// A single simple function to ensure crate compiles.
pub fn hello() -> &'static str {
    "hello"
}
