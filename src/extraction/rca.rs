use std::path::Path;

use rust_code_analysis::{get_function_spaces, FuncSpace, SpaceKind, LANG};
use thiserror::Error;

use crate::domain::metrics::{FunctionKind, FunctionMetrics};
use crate::extraction::MetricsExtractor;

#[derive(Debug, Error)]
pub enum RcaError {
    #[error("failed to read file: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("failed to parse file: {path}")]
    ParseError { path: String },
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
        collect_functions(&space, &mut functions);
        Ok(functions)
    }
}

/// Recursively collect function metrics from a FuncSpace tree.
fn collect_functions(space: &FuncSpace, out: &mut Vec<FunctionMetrics>) {
    // Only collect actual functions, not the file-level unit or impl blocks
    if let Some(kind) = to_function_kind(space.kind) {
        out.push(FunctionMetrics {
            name: space.name.clone(),
            start_line: space.start_line,
            end_line: space.end_line,
            kind,
            sloc: space.metrics.loc.sloc(),
            ploc: space.metrics.loc.ploc(),
            lloc: space.metrics.loc.lloc(),
            cloc: space.metrics.loc.cloc(),
            blank: space.metrics.loc.blank(),
            cyclomatic: space.metrics.cyclomatic.cyclomatic(),
            cognitive: space.metrics.cognitive.cognitive(),
            halstead_difficulty: space.metrics.halstead.difficulty(),
            halstead_effort: space.metrics.halstead.effort(),
            halstead_bugs: space.metrics.halstead.bugs(),
            mi: space.metrics.mi.mi_visual_studio(),
            nargs: space.metrics.nargs.fn_args() as usize,
            nexits: space.metrics.nexits.exit() as usize,
        });
    }

    // Recurse into children (functions inside impl blocks, nested functions, etc.)
    for child in &space.spaces {
        collect_functions(child, out);
    }
}

/// Map rac's SpaceKind to our FunctionKind, returning None for non-function spaces.
fn to_function_kind(kind: SpaceKind) -> Option<FunctionKind> {
    match kind {
        SpaceKind::Function => Some(FunctionKind::Function),
        // rac treats closures as functions, but we could distinguish if needed
        _ => None,
    }
}
