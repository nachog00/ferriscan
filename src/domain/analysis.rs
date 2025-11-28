use std::path::Path;

use walkdir::WalkDir;

use crate::domain::metrics::{CrateMetrics, FileMetrics, WorkspaceMetrics};
use crate::extraction::MetricsExtractor;
use crate::workspace::CrateInfo;

/// Analyze a single file using the provided extractor.
pub fn analyze_file<E: MetricsExtractor>(
    extractor: &E,
    path: &Path,
    relative_path: String,
) -> Result<FileMetrics, E::Error> {
    let functions = extractor.extract_functions(path)?;
    Ok(FileMetrics::from_functions(relative_path, functions))
}

/// Analyze all Rust files in a crate.
pub fn analyze_crate<E: MetricsExtractor>(
    extractor: &E,
    crate_info: &CrateInfo,
) -> CrateMetrics {
    let src_path = crate_info.path.join("src");

    if !src_path.exists() {
        return CrateMetrics::from_files(crate_info.name.clone(), Vec::new());
    }

    let mut files = Vec::new();

    for entry in WalkDir::new(&src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "rs"))
    {
        let relative_path = entry
            .path()
            .strip_prefix(&crate_info.path)
            .unwrap_or(entry.path())
            .display()
            .to_string();

        match analyze_file(extractor, entry.path(), relative_path) {
            Ok(file_metrics) => files.push(file_metrics),
            Err(_) => {
                // Skip files that fail to parse
                // TODO: consider collecting errors
            }
        }
    }

    CrateMetrics::from_files(crate_info.name.clone(), files)
}

/// Analyze an entire workspace.
pub fn analyze_workspace<E: MetricsExtractor>(
    extractor: &E,
    crates: &[CrateInfo],
) -> WorkspaceMetrics {
    let crate_metrics: Vec<CrateMetrics> = crates
        .iter()
        .map(|c| analyze_crate(extractor, c))
        .collect();

    WorkspaceMetrics::from_crates(crate_metrics)
}
