use serde::{Deserialize, Serialize};

use super::weighted_avg;
use super::file_metrics::FileMetrics;

/// Metrics for a crate, aggregated from its files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateMetrics {
    /// Crate name.
    pub name: String,

    /// All files in this crate.
    pub files: Vec<FileMetrics>,

    // Aggregated totals
    pub total_sloc: f64,
    pub total_ploc: f64,
    pub total_lloc: f64,
    pub total_cloc: f64,
    pub total_blank: f64,
    pub total_functions: usize,
    pub total_closures: usize,

    // Averages
    pub avg_file_sloc: f64,
    pub avg_functions_per_file: f64,
    pub avg_cyclomatic: f64,
    pub avg_cognitive: f64,
    pub avg_mi: f64,
    pub avg_halstead_difficulty: f64,
}

impl CrateMetrics {
    /// Construct crate metrics by aggregating from file metrics.
    pub fn from_files(name: String, files: Vec<FileMetrics>) -> Self {
        let n = files.len();

        if n == 0 {
            return Self {
                name,
                files,
                total_sloc: 0.0,
                total_ploc: 0.0,
                total_lloc: 0.0,
                total_cloc: 0.0,
                total_blank: 0.0,
                total_functions: 0,
                total_closures: 0,
                avg_file_sloc: 0.0,
                avg_functions_per_file: 0.0,
                avg_cyclomatic: 0.0,
                avg_cognitive: 0.0,
                avg_mi: 0.0,
                avg_halstead_difficulty: 0.0,
            };
        }

        let total_sloc: f64 = files.iter().map(|f| f.sloc).sum();
        let total_ploc: f64 = files.iter().map(|f| f.ploc).sum();
        let total_lloc: f64 = files.iter().map(|f| f.lloc).sum();
        let total_cloc: f64 = files.iter().map(|f| f.cloc).sum();
        let total_blank: f64 = files.iter().map(|f| f.blank).sum();
        let total_functions: usize = files.iter().map(|f| f.function_count()).sum();
        let total_closures: usize = files.iter().map(|f| f.closure_count()).sum();

        let n_f64 = n as f64;

        // Weighted averages by SLOC for complexity metrics
        let avg_cyclomatic = weighted_avg(&files, |f| f.cyclomatic_avg, |f| f.sloc);
        let avg_cognitive = weighted_avg(&files, |f| f.cognitive_avg, |f| f.sloc);
        let avg_mi = weighted_avg(&files, |f| f.mi_avg, |f| f.sloc);
        let avg_halstead_difficulty =
            weighted_avg(&files, |f| f.halstead_difficulty_avg, |f| f.sloc);

        Self {
            name,
            files,
            total_sloc,
            total_ploc,
            total_lloc,
            total_cloc,
            total_blank,
            total_functions,
            total_closures,
            avg_file_sloc: total_sloc / n_f64,
            avg_functions_per_file: total_functions as f64 / n_f64,
            avg_cyclomatic,
            avg_cognitive,
            avg_mi,
            avg_halstead_difficulty,
        }
    }

    /// Number of files in this crate.
    pub fn file_count(&self) -> usize {
        self.files.len()
    }
}
