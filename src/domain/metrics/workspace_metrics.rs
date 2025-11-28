use serde::{Deserialize, Serialize};

use crate::domain::primitive::sloc::Sloc;

use super::crate_metrics::CrateMetrics;
use super::weighted_avg;

/// Metrics for a workspace, aggregated from its crates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMetrics {
    /// All crates in this workspace.
    pub crates: Vec<CrateMetrics>,

    // Aggregated totals
    pub total_sloc: Sloc,
    pub total_functions: usize,

    // Averages
    pub avg_cyclomatic: f64,
    pub avg_cognitive: f64,
    pub avg_mi: f64,
}

impl WorkspaceMetrics {
    /// Construct workspace metrics by aggregating from crate metrics.
    pub fn from_crates(crates: Vec<CrateMetrics>) -> Self {
        if crates.is_empty() {
            return Self {
                crates,
                total_sloc: Sloc::zero(),
                total_functions: 0,
                avg_cyclomatic: 0.0,
                avg_cognitive: 0.0,
                avg_mi: 0.0,
            };
        }

        let total_sloc: Sloc = crates.iter().map(|c| c.total_sloc).sum();
        let total_functions: usize = crates.iter().map(|c| c.total_functions).sum();

        // Weighted averages by SLOC
        let avg_cyclomatic =
            weighted_avg(&crates, |c| c.avg_cyclomatic, |c| c.total_sloc.value());
        let avg_cognitive =
            weighted_avg(&crates, |c| c.avg_cognitive, |c| c.total_sloc.value());
        let avg_mi = weighted_avg(&crates, |c| c.avg_mi, |c| c.total_sloc.value());

        Self {
            crates,
            total_sloc,
            total_functions,
            avg_cyclomatic,
            avg_cognitive,
            avg_mi,
        }
    }

    /// Total number of crates.
    pub fn crate_count(&self) -> usize {
        self.crates.len()
    }

    /// Total number of files across all crates.
    pub fn file_count(&self) -> usize {
        self.crates.iter().map(|c| c.file_count()).sum()
    }
}
