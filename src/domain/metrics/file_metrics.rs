use serde::{Deserialize, Serialize};

use super::function_metrics::{FunctionKind, FunctionMetrics};

/// Metrics for a file, aggregated from its functions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetrics {
    /// Relative path of the file.
    pub path: String,

    /// All functions in this file.
    pub functions: Vec<FunctionMetrics>,

    // Aggregated size metrics
    pub sloc: f64,
    pub ploc: f64,
    pub lloc: f64,
    pub cloc: f64,
    pub blank: f64,

    // Aggregated complexity
    pub cyclomatic_sum: f64,
    pub cyclomatic_avg: f64,
    pub cognitive_sum: f64,
    pub cognitive_avg: f64,

    // Aggregated Halstead
    pub halstead_difficulty_avg: f64,
    pub halstead_effort_sum: f64,
    pub halstead_bugs_sum: f64,

    /// Average maintainability index.
    pub mi_avg: f64,

    // Counts
    pub nargs_sum: usize,
    pub nargs_avg: f64,
    pub nexits_sum: usize,
}

impl FileMetrics {
    /// Construct file metrics by aggregating from function metrics.
    pub fn from_functions(path: String, functions: Vec<FunctionMetrics>) -> Self {
        let n = functions.len();

        if n == 0 {
            return Self {
                path,
                functions,
                sloc: 0.0,
                ploc: 0.0,
                lloc: 0.0,
                cloc: 0.0,
                blank: 0.0,
                cyclomatic_sum: 0.0,
                cyclomatic_avg: 0.0,
                cognitive_sum: 0.0,
                cognitive_avg: 0.0,
                halstead_difficulty_avg: 0.0,
                halstead_effort_sum: 0.0,
                halstead_bugs_sum: 0.0,
                mi_avg: 0.0,
                nargs_sum: 0,
                nargs_avg: 0.0,
                nexits_sum: 0,
            };
        }

        let n_f64 = n as f64;

        let sloc: f64 = functions.iter().map(|f| f.sloc).sum();
        let ploc: f64 = functions.iter().map(|f| f.ploc).sum();
        let lloc: f64 = functions.iter().map(|f| f.lloc).sum();
        let cloc: f64 = functions.iter().map(|f| f.cloc).sum();
        let blank: f64 = functions.iter().map(|f| f.blank).sum();

        let cyclomatic_sum: f64 = functions.iter().map(|f| f.cyclomatic).sum();
        let cognitive_sum: f64 = functions.iter().map(|f| f.cognitive).sum();
        let halstead_difficulty_avg: f64 =
            functions.iter().map(|f| f.halstead_difficulty).sum::<f64>() / n_f64;
        let halstead_effort_sum: f64 = functions.iter().map(|f| f.halstead_effort).sum();
        let halstead_bugs_sum: f64 = functions.iter().map(|f| f.halstead_bugs).sum();
        let mi_avg: f64 = functions.iter().map(|f| f.mi).sum::<f64>() / n_f64;

        let nargs_sum: usize = functions.iter().map(|f| f.nargs).sum();
        let nexits_sum: usize = functions.iter().map(|f| f.nexits).sum();

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
            mi_avg,
            nargs_sum,
            nargs_avg: nargs_sum as f64 / n_f64,
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
