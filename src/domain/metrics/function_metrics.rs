use serde::{Deserialize, Serialize};

/// The kind of function-like construct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionKind {
    Function,
    Closure,
}

/// Metrics for a single function, method, or closure.
///
/// This is the fundamental unit of measurement. All higher-level metrics
/// (file, crate, workspace) are aggregated from function metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionMetrics {
    /// Function name (if available).
    pub name: Option<String>,

    /// Start line in the file (1-indexed).
    pub start_line: usize,

    /// End line in the file (1-indexed).
    pub end_line: usize,

    /// What kind of function-like construct this is.
    pub kind: FunctionKind,

    // Size metrics
    /// Source lines of code.
    pub sloc: f64,

    /// Physical lines of code.
    pub ploc: f64,

    /// Logical lines of code.
    pub lloc: f64,

    /// Comment lines of code.
    pub cloc: f64,

    /// Blank lines.
    pub blank: f64,

    // Complexity metrics
    /// Cyclomatic complexity.
    pub cyclomatic: f64,

    /// Cognitive complexity.
    pub cognitive: f64,

    // Halstead metrics
    pub halstead_difficulty: f64,
    pub halstead_effort: f64,
    pub halstead_bugs: f64,

    /// Maintainability index (Visual Studio variant).
    pub mi: f64,

    /// Number of arguments.
    pub nargs: usize,

    /// Number of exit points.
    pub nexits: usize,
}
