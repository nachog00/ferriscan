use rust_code_analysis::{get_function_spaces, FuncSpace, LANG};
use std::path::Path;
use walkdir::WalkDir;

use crate::report::{CrateReport, FileMetrics, WorkspaceReport};
use crate::workspace::CrateInfo;

/// Analyze a single Rust source file and return its FuncSpace metrics.
pub fn analyze_file(path: &Path) -> Option<FuncSpace> {
    let source = std::fs::read(path).ok()?;
    get_function_spaces(&LANG::Rust, source, path, None)
}

/// Count items in a FuncSpace tree by kind.
fn count_space_kinds(space: &FuncSpace) -> ItemCounts {
    let mut counts = ItemCounts::default();
    count_recursive(space, &mut counts);
    counts
}

fn count_recursive(space: &FuncSpace, counts: &mut ItemCounts) {
    use rust_code_analysis::SpaceKind;

    match space.kind {
        SpaceKind::Function => counts.functions += 1,
        SpaceKind::Unit => counts.units += 1,
        _ => counts.other += 1,
    }

    for child in &space.spaces {
        count_recursive(child, counts);
    }
}

#[derive(Debug, Default, Clone)]
pub struct ItemCounts {
    pub functions: usize,
    pub units: usize,
    pub other: usize,
}

/// Analyze all Rust files in a crate directory.
pub fn analyze_crate(crate_info: &CrateInfo) -> CrateReport {
    let mut report = CrateReport {
        name: crate_info.name.clone(),
        path: crate_info.path.clone(),
        ..Default::default()
    };

    let src_path = crate_info.path.join("src");
    if !src_path.exists() {
        return report;
    }

    let mut all_metrics: Vec<FileMetrics> = Vec::new();

    for entry in WalkDir::new(&src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        if let Some(space) = analyze_file(entry.path()) {
            let relative_path = entry
                .path()
                .strip_prefix(&crate_info.path)
                .unwrap_or(entry.path())
                .display()
                .to_string();

            let counts = count_space_kinds(&space);

            let file_metrics = FileMetrics {
                path: relative_path,
                sloc: space.metrics.loc.sloc(),
                ploc: space.metrics.loc.ploc(),
                lloc: space.metrics.loc.lloc(),
                cloc: space.metrics.loc.cloc(),
                blank: space.metrics.loc.blank(),
                functions: space.metrics.nom.functions(),
                closures: space.metrics.nom.closures(),
                cyclomatic_sum: space.metrics.cyclomatic.cyclomatic_sum(),
                cyclomatic_avg: space.metrics.cyclomatic.cyclomatic_average(),
                cognitive_sum: space.metrics.cognitive.cognitive_sum(),
                cognitive_avg: space.metrics.cognitive.cognitive_average(),
                halstead_difficulty: space.metrics.halstead.difficulty(),
                halstead_effort: space.metrics.halstead.effort(),
                halstead_bugs: space.metrics.halstead.bugs(),
                mi_visual_studio: space.metrics.mi.mi_visual_studio(),
                mi_sei: space.metrics.mi.mi_sei(),
                mi_original: space.metrics.mi.mi_original(),
                nargs_total: space.metrics.nargs.nargs_total(),
                nargs_avg: space.metrics.nargs.nargs_average(),
                nexits_sum: space.metrics.nexits.exit_sum(),
                item_counts: counts,
            };

            all_metrics.push(file_metrics);
        }
    }

    // Compute aggregates
    report.file_count = all_metrics.len();

    if !all_metrics.is_empty() {
        report.total_sloc = all_metrics.iter().map(|m| m.sloc).sum();
        report.total_ploc = all_metrics.iter().map(|m| m.ploc).sum();
        report.total_lloc = all_metrics.iter().map(|m| m.lloc).sum();
        report.total_cloc = all_metrics.iter().map(|m| m.cloc).sum();
        report.total_blank = all_metrics.iter().map(|m| m.blank).sum();
        report.total_functions = all_metrics.iter().map(|m| m.functions).sum();
        report.total_closures = all_metrics.iter().map(|m| m.closures).sum();

        // Weighted averages (weighted by SLOC)
        let total_sloc = report.total_sloc;
        if total_sloc > 0.0 {
            report.avg_cyclomatic = all_metrics
                .iter()
                .map(|m| m.cyclomatic_avg * m.sloc)
                .sum::<f64>()
                / total_sloc;

            report.avg_cognitive = all_metrics
                .iter()
                .map(|m| m.cognitive_avg * m.sloc)
                .sum::<f64>()
                / total_sloc;

            report.avg_mi = all_metrics
                .iter()
                .map(|m| m.mi_visual_studio * m.sloc)
                .sum::<f64>()
                / total_sloc;

            report.avg_halstead_difficulty = all_metrics
                .iter()
                .map(|m| m.halstead_difficulty * m.sloc)
                .sum::<f64>()
                / total_sloc;
        }

        // Simple averages
        let n = all_metrics.len() as f64;
        report.avg_file_sloc = report.total_sloc / n;
        report.avg_functions_per_file = report.total_functions / n;

        // Find largest files
        let mut sorted_by_sloc = all_metrics.clone();
        sorted_by_sloc.sort_by(|a, b| b.sloc.partial_cmp(&a.sloc).unwrap_or(std::cmp::Ordering::Equal));
        report.largest_files = sorted_by_sloc.into_iter().take(10).collect();

        // Find most complex files (by cyclomatic)
        let mut sorted_by_complexity = all_metrics.clone();
        sorted_by_complexity.sort_by(|a, b| {
            b.cyclomatic_sum
                .partial_cmp(&a.cyclomatic_sum)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        report.most_complex_files = sorted_by_complexity.into_iter().take(10).collect();

        // Find files with worst maintainability (lowest MI)
        let mut sorted_by_mi = all_metrics.clone();
        sorted_by_mi.sort_by(|a, b| {
            a.mi_visual_studio
                .partial_cmp(&b.mi_visual_studio)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        report.worst_maintainability = sorted_by_mi.into_iter().take(10).collect();

        report.files = all_metrics;
    }

    report
}

/// Analyze an entire workspace.
pub fn analyze_workspace(crates: &[CrateInfo]) -> WorkspaceReport {
    let mut report = WorkspaceReport::default();

    for crate_info in crates {
        let crate_report = analyze_crate(crate_info);
        report.crates.push(crate_report);
    }

    // Compute workspace-level aggregates
    report.total_crates = report.crates.len();
    report.total_files = report.crates.iter().map(|c| c.file_count).sum();
    report.total_sloc = report.crates.iter().map(|c| c.total_sloc).sum();
    report.total_functions = report.crates.iter().map(|c| c.total_functions).sum();

    if report.total_sloc > 0.0 {
        report.avg_cyclomatic = report
            .crates
            .iter()
            .map(|c| c.avg_cyclomatic * c.total_sloc)
            .sum::<f64>()
            / report.total_sloc;

        report.avg_cognitive = report
            .crates
            .iter()
            .map(|c| c.avg_cognitive * c.total_sloc)
            .sum::<f64>()
            / report.total_sloc;

        report.avg_mi = report
            .crates
            .iter()
            .map(|c| c.avg_mi * c.total_sloc)
            .sum::<f64>()
            / report.total_sloc;
    }

    // Sort crates by size
    report.crates.sort_by(|a, b| {
        b.total_sloc
            .partial_cmp(&a.total_sloc)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    report
}
