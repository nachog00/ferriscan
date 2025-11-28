use std::path::Path;

use rust_code_analysis::{get_function_spaces, FuncSpace, SpaceKind, LANG};
use thiserror::Error;

use crate::domain::metrics::function_metrics::{FunctionKind, FunctionMetrics};
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
use crate::extraction::MetricsExtractor;

#[derive(Debug, Error)]
pub enum RcaError {
    #[error("failed to read file: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("failed to parse file: {path}")]
    ParseError { path: String },

    #[error("invalid metric value: {0}")]
    InvalidMetric(String),
}

/// Metrics extractor using rust-code-analysis.
pub struct RcaExtractor;

impl MetricsExtractor for RcaExtractor {
    type Error = RcaError;

    fn extract_functions(&self, path: &Path) -> Result<Vec<FunctionMetrics>, Self::Error> {
        let source = std::fs::read(path)?;

        let space = get_function_spaces(&LANG::Rust, source, path, None).ok_or_else(|| {
            RcaError::ParseError {
                path: path.display().to_string(),
            }
        })?;

        let mut functions = Vec::new();
        collect_functions(&space, &mut functions)?;
        Ok(functions)
    }
}

/// Recursively collect function metrics from a FuncSpace tree.
fn collect_functions(space: &FuncSpace, out: &mut Vec<FunctionMetrics>) -> Result<(), RcaError> {
    // Only collect actual functions, not the file-level unit or impl blocks
    if let Some(kind) = to_function_kind(space.kind) {
        let metrics = build_function_metrics(space, kind)?;
        out.push(metrics);
    }

    // Recurse into children (functions inside impl blocks, nested functions, etc.)
    for child in &space.spaces {
        collect_functions(child, out)?;
    }

    Ok(())
}

/// Build FunctionMetrics from a FuncSpace, validating all primitive values.
fn build_function_metrics(space: &FuncSpace, kind: FunctionKind) -> Result<FunctionMetrics, RcaError> {
    let start_line = LineNumber::new(space.start_line)
        .map_err(|e| RcaError::InvalidMetric(format!("start_line: {}", e)))?;
    let end_line = LineNumber::new(space.end_line)
        .map_err(|e| RcaError::InvalidMetric(format!("end_line: {}", e)))?;

    let sloc = Sloc::new(space.metrics.loc.sloc())
        .map_err(|e| RcaError::InvalidMetric(format!("sloc: {}", e)))?;
    let ploc = Ploc::new(space.metrics.loc.ploc())
        .map_err(|e| RcaError::InvalidMetric(format!("ploc: {}", e)))?;
    let lloc = Lloc::new(space.metrics.loc.lloc())
        .map_err(|e| RcaError::InvalidMetric(format!("lloc: {}", e)))?;
    let cloc = Cloc::new(space.metrics.loc.cloc())
        .map_err(|e| RcaError::InvalidMetric(format!("cloc: {}", e)))?;
    let blank = Blank::new(space.metrics.loc.blank())
        .map_err(|e| RcaError::InvalidMetric(format!("blank: {}", e)))?;

    let cyclomatic = CyclomaticComplexity::new(space.metrics.cyclomatic.cyclomatic())
        .map_err(|e| RcaError::InvalidMetric(format!("cyclomatic: {}", e)))?;
    let cognitive = CognitiveComplexity::new(space.metrics.cognitive.cognitive())
        .map_err(|e| RcaError::InvalidMetric(format!("cognitive: {}", e)))?;

    let halstead_difficulty = HalsteadDifficulty::new(space.metrics.halstead.difficulty())
        .map_err(|e| RcaError::InvalidMetric(format!("halstead_difficulty: {}", e)))?;
    let halstead_effort = HalsteadEffort::new(space.metrics.halstead.effort())
        .map_err(|e| RcaError::InvalidMetric(format!("halstead_effort: {}", e)))?;
    let halstead_bugs = HalsteadBugs::new(space.metrics.halstead.bugs())
        .map_err(|e| RcaError::InvalidMetric(format!("halstead_bugs: {}", e)))?;

    let mi = MaintainabilityIndex::new(space.metrics.mi.mi_visual_studio())
        .map_err(|e| RcaError::InvalidMetric(format!("mi: {}", e)))?;

    // These are infallible for usize
    let nargs = Nargs::from(space.metrics.nargs.fn_args() as usize);
    let nexits = Nexits::from(space.metrics.nexits.exit() as usize);

    Ok(FunctionMetrics {
        name: space.name.clone(),
        start_line,
        end_line,
        kind,
        sloc,
        ploc,
        lloc,
        cloc,
        blank,
        cyclomatic,
        cognitive,
        halstead_difficulty,
        halstead_effort,
        halstead_bugs,
        mi,
        nargs,
        nexits,
    })
}

/// Map rac's SpaceKind to our FunctionKind, returning None for non-function spaces.
fn to_function_kind(kind: SpaceKind) -> Option<FunctionKind> {
    match kind {
        SpaceKind::Function => Some(FunctionKind::Function),
        // rac treats closures as functions, but we could distinguish if needed
        _ => None,
    }
}
