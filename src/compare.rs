use crate::report::{ComparisonReport, MetricsDelta, WorkspaceReport};

/// Compare two workspace reports.
pub fn compare_workspaces(
    left_name: &str,
    left: &WorkspaceReport,
    right_name: &str,
    right: &WorkspaceReport,
) -> ComparisonReport {
    let left_summary = left.to_summary();
    let right_summary = right.to_summary();

    let sloc_diff = right_summary.total_sloc - left_summary.total_sloc;
    let sloc_pct = if left_summary.total_sloc > 0.0 {
        (sloc_diff / left_summary.total_sloc) * 100.0
    } else {
        0.0
    };

    let delta = MetricsDelta {
        sloc_diff,
        sloc_pct,
        files_diff: right_summary.total_files - left_summary.total_files,
        functions_diff: right_summary.total_functions - left_summary.total_functions,
        cyclomatic_diff: right_summary.avg_cyclomatic - left_summary.avg_cyclomatic,
        cognitive_diff: right_summary.avg_cognitive - left_summary.avg_cognitive,
        mi_diff: right_summary.avg_mi - left_summary.avg_mi,
        sloc_per_file_diff: right_summary.sloc_per_file - left_summary.sloc_per_file,
        functions_per_file_diff: right_summary.functions_per_file - left_summary.functions_per_file,
    };

    ComparisonReport {
        left_name: left_name.to_string(),
        right_name: right_name.to_string(),
        left: left_summary,
        right: right_summary,
        delta,
    }
}
