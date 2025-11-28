use serde::{Deserialize, Serialize};

use crate::domain::primitive::blank::Blank;
use crate::domain::primitive::cloc::Cloc;
use crate::domain::primitive::cognitive::CognitiveComplexity;
use crate::domain::primitive::counts::{LineNumber, Nargs, Nexits};
use crate::domain::primitive::cyclomatic::CyclomaticComplexity;
use crate::domain::primitive::halstead::{HalsteadBugs, HalsteadDifficulty, HalsteadEffort};
use crate::domain::primitive::lloc::Lloc;
use crate::domain::primitive::mi::MaintainabilityIndex;
use crate::domain::primitive::ploc::Ploc;
use crate::domain::primitive::sloc::Sloc;

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
    pub start_line: LineNumber,

    /// End line in the file (1-indexed).
    pub end_line: LineNumber,

    /// What kind of function-like construct this is.
    pub kind: FunctionKind,

    // Size metrics
    /// Source lines of code.
    pub sloc: Sloc,

    /// Physical lines of code.
    pub ploc: Ploc,

    /// Logical lines of code.
    pub lloc: Lloc,

    /// Comment lines of code.
    pub cloc: Cloc,

    /// Blank lines.
    pub blank: Blank,

    // Complexity metrics
    /// Cyclomatic complexity.
    pub cyclomatic: CyclomaticComplexity,

    /// Cognitive complexity.
    pub cognitive: CognitiveComplexity,

    // Halstead metrics
    pub halstead_difficulty: HalsteadDifficulty,
    pub halstead_effort: HalsteadEffort,
    pub halstead_bugs: HalsteadBugs,

    /// Maintainability index (Visual Studio variant).
    pub mi: MaintainabilityIndex,

    /// Number of arguments.
    pub nargs: Nargs,

    /// Number of exit points.
    pub nexits: Nexits,
}
