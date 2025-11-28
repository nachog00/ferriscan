use serde::{Deserialize, Serialize};

use crate::domain::primitive::blank::Blank;
use crate::domain::primitive::cloc::Cloc;
use crate::domain::primitive::counts::{Nargs, Nexits};
use crate::domain::primitive::lloc::Lloc;
use crate::domain::primitive::mi::MaintainabilityIndex;
use crate::domain::primitive::ploc::Ploc;
use crate::domain::primitive::sloc::Sloc;

use super::function_metrics::{FunctionKind, FunctionMetrics};

/// Metrics for a file, aggregated from its functions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetrics {
    /// Relative path of the file.
    pub path: String,

    /// All functions in this file.
    pub functions: Vec<FunctionMetrics>,

    // Aggregated size metrics
    pub sloc: Sloc,
    pub ploc: Ploc,
    pub lloc: Lloc,
    pub cloc: Cloc,
    pub blank: Blank,

    // Aggregated complexity
    pub cyclomatic_sum: f64,
    pub cyclomatic_avg: f64,
    pub cognitive_sum: f64,
    pub cognitive_avg: f64,

    // Aggregated Halstead
    pub halstead_difficulty_avg: f64,
    pub halstead_effort_sum: f64,
    pub halstead_bugs_sum: f64,

    /// File-level maintainability index (from whole file analysis, not averaged).
    pub mi: MaintainabilityIndex,

    // Counts
    pub nargs_sum: Nargs,
    pub nargs_avg: f64,
    pub nexits_sum: Nexits,
}

impl FileMetrics {
    /// Construct file metrics by aggregating from function metrics.
    /// The `file_mi` should come from file-level analysis, not averaged from functions.
    pub fn from_functions(
        path: String,
        functions: Vec<FunctionMetrics>,
        file_mi: MaintainabilityIndex,
    ) -> Self {
        let n = functions.len();

        if n == 0 {
            return Self {
                path,
                functions,
                sloc: Sloc::zero(),
                ploc: Ploc::zero(),
                lloc: Lloc::zero(),
                cloc: Cloc::zero(),
                blank: Blank::zero(),
                cyclomatic_sum: 0.0,
                cyclomatic_avg: 0.0,
                cognitive_sum: 0.0,
                cognitive_avg: 0.0,
                halstead_difficulty_avg: 0.0,
                halstead_effort_sum: 0.0,
                halstead_bugs_sum: 0.0,
                mi: file_mi,
                nargs_sum: Nargs::zero(),
                nargs_avg: 0.0,
                nexits_sum: Nexits::zero(),
            };
        }

        let n_f64 = n as f64;

        let sloc: Sloc = functions.iter().map(|f| f.sloc).sum();
        let ploc: Ploc = functions.iter().map(|f| f.ploc).sum();
        let lloc: Lloc = functions.iter().map(|f| f.lloc).sum();
        let cloc: Cloc = functions.iter().map(|f| f.cloc).sum();
        let blank: Blank = functions.iter().map(|f| f.blank).sum();

        let cyclomatic_sum: f64 = functions.iter().map(|f| f.cyclomatic.value()).sum();
        let cognitive_sum: f64 = functions.iter().map(|f| f.cognitive.value()).sum();
        let halstead_difficulty_avg: f64 =
            functions.iter().map(|f| f.halstead_difficulty.value()).sum::<f64>() / n_f64;
        let halstead_effort_sum: f64 = functions.iter().map(|f| f.halstead_effort.value()).sum();
        let halstead_bugs_sum: f64 = functions.iter().map(|f| f.halstead_bugs.value()).sum();

        let nargs_sum: Nargs = functions.iter().map(|f| f.nargs).sum();
        let nexits_sum: Nexits = functions.iter().map(|f| f.nexits).sum();

        Self {
            path,
            functions,
            sloc,
            ploc,
            lloc,
            cloc,
            blank,
            cyclomatic_sum,
            cyclomatic_avg: cyclomatic_sum / n_f64,
            cognitive_sum,
            cognitive_avg: cognitive_sum / n_f64,
            halstead_difficulty_avg,
            halstead_effort_sum,
            halstead_bugs_sum,
            mi: file_mi,
            nargs_sum,
            nargs_avg: nargs_sum.value() as f64 / n_f64,
            nexits_sum,
        }
    }

    /// Number of functions in this file.
    pub fn function_count(&self) -> usize {
        self.functions.len()
    }

    /// Number of closures in this file.
    pub fn closure_count(&self) -> usize {
        self.functions
            .iter()
            .filter(|f| f.kind == FunctionKind::Closure)
            .count()
    }
}
