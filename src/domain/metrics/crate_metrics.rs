use serde::{Deserialize, Serialize};

use crate::domain::primitive::blank::Blank;
use crate::domain::primitive::cloc::Cloc;
use crate::domain::primitive::lloc::Lloc;
use crate::domain::primitive::ploc::Ploc;
use crate::domain::primitive::sloc::Sloc;

use super::file_metrics::FileMetrics;
use super::weighted_avg;

/// Metrics for a crate, aggregated from its files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateMetrics {
    /// Crate name.
    pub name: String,

    /// All files in this crate.
    pub files: Vec<FileMetrics>,

    // Aggregated totals
    pub total_sloc: Sloc,
    pub total_ploc: Ploc,
    pub total_lloc: Lloc,
    pub total_cloc: Cloc,
    pub total_blank: Blank,
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
                total_sloc: Sloc::zero(),
                total_ploc: Ploc::zero(),
                total_lloc: Lloc::zero(),
                total_cloc: Cloc::zero(),
                total_blank: Blank::zero(),
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

        let total_sloc: Sloc = files.iter().map(|f| f.sloc).sum();
        let total_ploc: Ploc = files.iter().map(|f| f.ploc).sum();
        let total_lloc: Lloc = files.iter().map(|f| f.lloc).sum();
        let total_cloc: Cloc = files.iter().map(|f| f.cloc).sum();
        let total_blank: Blank = files.iter().map(|f| f.blank).sum();
        let total_functions: usize = files.iter().map(|f| f.function_count()).sum();
        let total_closures: usize = files.iter().map(|f| f.closure_count()).sum();

        let n_f64 = n as f64;

        // Weighted averages by SLOC for complexity metrics
        let avg_cyclomatic =
            weighted_avg(&files, |f| f.cyclomatic_avg, |f| f.sloc.value());
        let avg_cognitive = weighted_avg(&files, |f| f.cognitive_avg, |f| f.sloc.value());
        let avg_mi = weighted_avg(&files, |f| f.mi.value(), |f| f.sloc.value());
        let avg_halstead_difficulty =
            weighted_avg(&files, |f| f.halstead_difficulty_avg, |f| f.sloc.value());

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
            avg_file_sloc: total_sloc.value() / n_f64,
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
