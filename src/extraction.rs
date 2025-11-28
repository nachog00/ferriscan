pub mod rca;

use std::path::Path;

use crate::domain::metrics::function_metrics::FunctionMetrics;
use crate::domain::primitive::mi::MaintainabilityIndex;

/// Result of extracting metrics from a single file.
/// Contains both function-level metrics and file-level metrics that
/// can't be derived from function aggregation (like MI).
pub struct FileExtraction {
    /// All functions found in the file.
    pub functions: Vec<FunctionMetrics>,
    /// File-level maintainability index (from the whole file, not averaged from functions).
    pub file_mi: MaintainabilityIndex,
}

/// Abstraction over code parsing. The implementation (e.g., rust-code-analysis) is an internal detail.
pub trait MetricsExtractor {
    type Error: std::error::Error;

    /// Extract metrics from a source file.
    /// Returns both function-level metrics and file-level metrics.
    fn extract_file(&self, path: &Path) -> Result<FileExtraction, Self::Error>;
}
