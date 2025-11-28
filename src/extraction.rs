pub mod rca;

use std::path::Path;

use crate::domain::metrics::FunctionMetrics;

/// Abstraction over code parsing. The implementation (e.g., rust-code-analysis) is an internal detail.
pub trait MetricsExtractor {
    type Error: std::error::Error;

    /// Extract all function-level metrics from a source file.
    fn extract_functions(&self, path: &Path) -> Result<Vec<FunctionMetrics>, Self::Error>;
}
