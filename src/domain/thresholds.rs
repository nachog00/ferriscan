use serde::{Deserialize, Serialize};

use crate::domain::metrics::crate_metrics::CrateMetrics;
use crate::domain::metrics::file_metrics::FileMetrics;
use crate::domain::metrics::function_metrics::FunctionMetrics;

/// Thresholds for metric warnings.
#[derive(Debug, Clone)]
pub struct Thresholds {
    pub max_file_sloc: f64,
    pub max_cyclomatic: f64,
    pub max_cognitive: f64,
    pub min_mi: f64,
    pub max_functions_per_file: usize,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            max_file_sloc: 500.0,
            max_cyclomatic: 10.0,
            max_cognitive: 15.0,
            min_mi: 20.0,
            max_functions_per_file: 20,
        }
    }
}

/// A warning about a metric threshold violation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warning {
    pub file: String,
    pub function: Option<String>,
    pub line: Option<usize>,
    pub metric: String,
    pub value: f64,
    pub threshold: f64,
    pub message: String,
}

/// Check thresholds at file level.
pub fn check_file_thresholds(file: &FileMetrics, thresholds: &Thresholds) -> Vec<Warning> {
    let mut warnings = Vec::new();

    if file.sloc > thresholds.max_file_sloc {
        warnings.push(Warning {
            file: file.path.clone(),
            function: None,
            line: None,
            metric: "sloc".to_string(),
            value: file.sloc,
            threshold: thresholds.max_file_sloc,
            message: format!(
                "File has {} SLOC (threshold: {})",
                file.sloc as i64, thresholds.max_file_sloc as i64
            ),
        });
    }

    if file.function_count() > thresholds.max_functions_per_file {
        warnings.push(Warning {
            file: file.path.clone(),
            function: None,
            line: None,
            metric: "functions".to_string(),
            value: file.function_count() as f64,
            threshold: thresholds.max_functions_per_file as f64,
            message: format!(
                "Too many functions: {} (threshold: {})",
                file.function_count(),
                thresholds.max_functions_per_file
            ),
        });
    }

    warnings
}

/// Check thresholds at function level.
pub fn check_function_thresholds(
    file_path: &str,
    func: &FunctionMetrics,
    thresholds: &Thresholds,
) -> Vec<Warning> {
    let mut warnings = Vec::new();

    if func.cyclomatic > thresholds.max_cyclomatic {
        warnings.push(Warning {
            file: file_path.to_string(),
            function: func.name.clone(),
            line: Some(func.start_line),
            metric: "cyclomatic".to_string(),
            value: func.cyclomatic,
            threshold: thresholds.max_cyclomatic,
            message: format!(
                "High cyclomatic complexity: {:.1} (threshold: {:.1})",
                func.cyclomatic, thresholds.max_cyclomatic
            ),
        });
    }

    if func.cognitive > thresholds.max_cognitive {
        warnings.push(Warning {
            file: file_path.to_string(),
            function: func.name.clone(),
            line: Some(func.start_line),
            metric: "cognitive".to_string(),
            value: func.cognitive,
            threshold: thresholds.max_cognitive,
            message: format!(
                "High cognitive complexity: {:.1} (threshold: {:.1})",
                func.cognitive, thresholds.max_cognitive
            ),
        });
    }

    if func.mi < thresholds.min_mi && func.mi > 0.0 {
        warnings.push(Warning {
            file: file_path.to_string(),
            function: func.name.clone(),
            line: Some(func.start_line),
            metric: "maintainability".to_string(),
            value: func.mi,
            threshold: thresholds.min_mi,
            message: format!(
                "Low maintainability index: {:.1} (threshold: {:.1})",
                func.mi, thresholds.min_mi
            ),
        });
    }

    warnings
}

/// Check all thresholds for a crate.
pub fn check_crate_thresholds(crate_metrics: &CrateMetrics, thresholds: &Thresholds) -> Vec<Warning> {
    let mut warnings = Vec::new();

    for file in &crate_metrics.files {
        warnings.extend(check_file_thresholds(file, thresholds));

        for func in &file.functions {
            warnings.extend(check_function_thresholds(&file.path, func, thresholds));
        }
    }

    warnings
}
