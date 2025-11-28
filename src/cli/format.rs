use clap::ValueEnum;
use comfy_table::{presets::UTF8_FULL, Cell, Color, ContentArrangement, Table};

use ferriscan::domain::metrics::workspace_metrics::WorkspaceMetrics;

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
    JsonPretty,
}

pub fn format_workspace(workspace: &WorkspaceMetrics, format: &OutputFormat, verbose: bool) -> String {
    match format {
        OutputFormat::Json => serde_json::to_string(workspace).unwrap_or_default(),
        OutputFormat::JsonPretty => serde_json::to_string_pretty(workspace).unwrap_or_default(),
        OutputFormat::Table => format_workspace_table(workspace, verbose),
    }
}

fn format_workspace_table(workspace: &WorkspaceMetrics, verbose: bool) -> String {
    let mut output = String::new();

    // Summary table
    output.push_str("\n=== Workspace Summary ===\n\n");

    let mut summary = Table::new();
    summary.load_preset(UTF8_FULL);
    summary.set_content_arrangement(ContentArrangement::Dynamic);
    summary.set_header(vec!["Metric", "Value"]);
    summary.add_row(vec!["Total Crates", &workspace.crate_count().to_string()]);
    summary.add_row(vec!["Total Files", &workspace.file_count().to_string()]);
    summary.add_row(vec!["Total SLOC", &format!("{:.0}", workspace.total_sloc.value())]);
    summary.add_row(vec![
        "Total Functions",
        &workspace.total_functions.to_string(),
    ]);
    summary.add_row(vec![
        "Avg Cyclomatic",
        &format!("{:.2}", workspace.avg_cyclomatic),
    ]);
    summary.add_row(vec!["Avg Cognitive", &format!("{:.2}", workspace.avg_cognitive)]);
    summary.add_row(vec!["Avg Maintainability", &format!("{:.1}", workspace.avg_mi)]);

    let file_count = workspace.file_count();
    if file_count > 0 {
        summary.add_row(vec![
            "SLOC per File",
            &format!("{:.1}", workspace.total_sloc.value() / file_count as f64),
        ]);
        summary.add_row(vec![
            "Functions per File",
            &format!("{:.1}", workspace.total_functions as f64 / file_count as f64),
        ]);
    }

    output.push_str(&summary.to_string());
    output.push_str("\n\n");

    // Crates table
    output.push_str("=== Crates by Size ===\n\n");

    let mut crates_table = Table::new();
    crates_table.load_preset(UTF8_FULL);
    crates_table.set_content_arrangement(ContentArrangement::Dynamic);
    crates_table.set_header(vec!["Crate", "Files", "SLOC", "Funcs", "Cyc", "Cog", "MI"]);

    for crate_metrics in &workspace.crates {
        let mi_color = mi_color(crate_metrics.avg_mi);

        crates_table.add_row(vec![
            Cell::new(&crate_metrics.name),
            Cell::new(crate_metrics.file_count()),
            Cell::new(format!("{:.0}", crate_metrics.total_sloc.value())),
            Cell::new(crate_metrics.total_functions),
            Cell::new(format!("{:.1}", crate_metrics.avg_cyclomatic)),
            Cell::new(format!("{:.1}", crate_metrics.avg_cognitive)),
            Cell::new(format!("{:.1}", crate_metrics.avg_mi)).fg(mi_color),
        ]);
    }

    output.push_str(&crates_table.to_string());

    if verbose {
        output.push_str(&format_verbose_details(workspace));
    }

    output
}

fn format_verbose_details(workspace: &WorkspaceMetrics) -> String {
    let mut output = String::new();

    // Collect all files with crate names
    let all_files: Vec<_> = workspace
        .crates
        .iter()
        .flat_map(|c| c.files.iter().map(move |f| (&c.name, f)))
        .collect();

    // Largest files
    output.push_str("\n\n=== Largest Files (Top 10) ===\n\n");

    let mut by_sloc = all_files.clone();
    by_sloc.sort_by(|a, b| b.1.sloc.value().partial_cmp(&a.1.sloc.value()).unwrap_or(std::cmp::Ordering::Equal));
    by_sloc.truncate(10);

    let mut files_table = Table::new();
    files_table.load_preset(UTF8_FULL);
    files_table.set_content_arrangement(ContentArrangement::Dynamic);
    files_table.set_header(vec!["Crate", "File", "SLOC", "Funcs", "Cyc", "MI"]);

    for (crate_name, file) in by_sloc {
        files_table.add_row(vec![
            crate_name.as_str(),
            &file.path,
            &format!("{:.0}", file.sloc.value()),
            &file.function_count().to_string(),
            &format!("{:.1}", file.cyclomatic_avg),
            &format!("{:.1}", file.mi_avg),
        ]);
    }

    output.push_str(&files_table.to_string());

    // Worst maintainability
    output.push_str("\n\n=== Worst Maintainability (Top 10) ===\n\n");

    let mut by_mi: Vec<_> = all_files
        .into_iter()
        .filter(|(_, f)| f.mi_avg > 0.0)
        .collect();
    by_mi.sort_by(|a, b| {
        a.1.mi_avg
            .partial_cmp(&b.1.mi_avg)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    by_mi.truncate(10);

    let mut worst_table = Table::new();
    worst_table.load_preset(UTF8_FULL);
    worst_table.set_content_arrangement(ContentArrangement::Dynamic);
    worst_table.set_header(vec!["Crate", "File", "MI", "SLOC", "Cyc"]);

    for (crate_name, file) in by_mi {
        worst_table.add_row(vec![
            Cell::new(crate_name.as_str()),
            Cell::new(&file.path),
            Cell::new(format!("{:.1}", file.mi_avg)).fg(mi_color(file.mi_avg)),
            Cell::new(format!("{:.0}", file.sloc.value())),
            Cell::new(format!("{:.1}", file.cyclomatic_avg)),
        ]);
    }

    output.push_str(&worst_table.to_string());

    output
}

fn mi_color(mi: f64) -> Color {
    if mi < 20.0 {
        Color::Red
    } else if mi < 40.0 {
        Color::Yellow
    } else {
        Color::Green
    }
}

// Comparison formatting


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComparisonReport {
    pub left_name: String,
    pub right_name: String,
    pub left: WorkspaceSummary,
    pub right: WorkspaceSummary,
    pub delta: MetricsDelta,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkspaceSummary {
    pub total_crates: usize,
    pub total_files: usize,
    pub total_sloc: f64,
    pub total_functions: usize,
    pub avg_cyclomatic: f64,
    pub avg_cognitive: f64,
    pub avg_mi: f64,
    pub sloc_per_file: f64,
    pub functions_per_file: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetricsDelta {
    pub sloc_diff: f64,
    pub sloc_pct: f64,
    pub files_diff: i64,
    pub functions_diff: i64,
    pub cyclomatic_diff: f64,
    pub cognitive_diff: f64,
    pub mi_diff: f64,
    pub sloc_per_file_diff: f64,
    pub functions_per_file_diff: f64,
}

fn workspace_to_summary(w: &WorkspaceMetrics) -> WorkspaceSummary {
    let file_count = w.file_count();
    WorkspaceSummary {
        total_crates: w.crate_count(),
        total_files: file_count,
        total_sloc: w.total_sloc.value(),
        total_functions: w.total_functions,
        avg_cyclomatic: w.avg_cyclomatic,
        avg_cognitive: w.avg_cognitive,
        avg_mi: w.avg_mi,
        sloc_per_file: if file_count > 0 {
            w.total_sloc.value() / file_count as f64
        } else {
            0.0
        },
        functions_per_file: if file_count > 0 {
            w.total_functions as f64 / file_count as f64
        } else {
            0.0
        },
    }
}

pub fn compare_workspaces(
    left_name: String,
    right_name: String,
    left: &WorkspaceMetrics,
    right: &WorkspaceMetrics,
) -> ComparisonReport {
    let left_summary = workspace_to_summary(left);
    let right_summary = workspace_to_summary(right);

    let delta = MetricsDelta {
        sloc_diff: right_summary.total_sloc - left_summary.total_sloc,
        sloc_pct: if left_summary.total_sloc > 0.0 {
            ((right_summary.total_sloc - left_summary.total_sloc) / left_summary.total_sloc) * 100.0
        } else {
            0.0
        },
        files_diff: right_summary.total_files as i64 - left_summary.total_files as i64,
        functions_diff: right_summary.total_functions as i64 - left_summary.total_functions as i64,
        cyclomatic_diff: right_summary.avg_cyclomatic - left_summary.avg_cyclomatic,
        cognitive_diff: right_summary.avg_cognitive - left_summary.avg_cognitive,
        mi_diff: right_summary.avg_mi - left_summary.avg_mi,
        sloc_per_file_diff: right_summary.sloc_per_file - left_summary.sloc_per_file,
        functions_per_file_diff: right_summary.functions_per_file - left_summary.functions_per_file,
    };

    ComparisonReport {
        left_name,
        right_name,
        left: left_summary,
        right: right_summary,
        delta,
    }
}

pub fn format_comparison(report: &ComparisonReport, format: &OutputFormat) -> String {
    match format {
        OutputFormat::Json => serde_json::to_string(report).unwrap_or_default(),
        OutputFormat::JsonPretty => serde_json::to_string_pretty(report).unwrap_or_default(),
        OutputFormat::Table => format_comparison_table(report),
    }
}

fn format_comparison_table(report: &ComparisonReport) -> String {
    let mut output = String::new();

    output.push_str(&format!(
        "\n=== Comparison: {} vs {} ===\n\n",
        report.left_name, report.right_name
    ));

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        "Metric",
        &report.left_name,
        &report.right_name,
        "Delta",
        "%",
    ]);

    let delta_cell = |diff: f64, higher_is_better: bool| {
        let color = if diff.abs() < 0.01 {
            Color::White
        } else if (diff > 0.0) == higher_is_better {
            Color::Green
        } else {
            Color::Red
        };
        Cell::new(format!("{:+.1}", diff)).fg(color)
    };

    let pct_cell = |pct: f64, higher_is_better: bool| {
        let color = if pct.abs() < 1.0 {
            Color::White
        } else if (pct > 0.0) == higher_is_better {
            Color::Green
        } else {
            Color::Red
        };
        Cell::new(format!("{:+.1}%", pct)).fg(color)
    };

    table.add_row(vec![
        Cell::new("Crates"),
        Cell::new(report.left.total_crates),
        Cell::new(report.right.total_crates),
        Cell::new(""),
        Cell::new(""),
    ]);

    table.add_row(vec![
        Cell::new("Files"),
        Cell::new(report.left.total_files),
        Cell::new(report.right.total_files),
        Cell::new(format!("{:+}", report.delta.files_diff)),
        Cell::new(""),
    ]);

    table.add_row(vec![
        Cell::new("SLOC"),
        Cell::new(format!("{:.0}", report.left.total_sloc)),
        Cell::new(format!("{:.0}", report.right.total_sloc)),
        delta_cell(report.delta.sloc_diff, false),
        pct_cell(report.delta.sloc_pct, false),
    ]);

    table.add_row(vec![
        Cell::new("Functions"),
        Cell::new(report.left.total_functions),
        Cell::new(report.right.total_functions),
        Cell::new(format!("{:+}", report.delta.functions_diff)),
        Cell::new(""),
    ]);

    table.add_row(vec![
        Cell::new("SLOC/File"),
        Cell::new(format!("{:.1}", report.left.sloc_per_file)),
        Cell::new(format!("{:.1}", report.right.sloc_per_file)),
        delta_cell(report.delta.sloc_per_file_diff, false),
        Cell::new(""),
    ]);

    table.add_row(vec![
        Cell::new("Funcs/File"),
        Cell::new(format!("{:.1}", report.left.functions_per_file)),
        Cell::new(format!("{:.1}", report.right.functions_per_file)),
        delta_cell(report.delta.functions_per_file_diff, false),
        Cell::new(""),
    ]);

    table.add_row(vec![
        Cell::new("Avg Cyclomatic"),
        Cell::new(format!("{:.2}", report.left.avg_cyclomatic)),
        Cell::new(format!("{:.2}", report.right.avg_cyclomatic)),
        delta_cell(report.delta.cyclomatic_diff, false),
        Cell::new(""),
    ]);

    table.add_row(vec![
        Cell::new("Avg Cognitive"),
        Cell::new(format!("{:.2}", report.left.avg_cognitive)),
        Cell::new(format!("{:.2}", report.right.avg_cognitive)),
        delta_cell(report.delta.cognitive_diff, false),
        Cell::new(""),
    ]);

    table.add_row(vec![
        Cell::new("Avg MI"),
        Cell::new(format!("{:.1}", report.left.avg_mi)),
        Cell::new(format!("{:.1}", report.right.avg_mi)),
        delta_cell(report.delta.mi_diff, true), // higher MI is better
        Cell::new(""),
    ]);

    output.push_str(&table.to_string());
    output
}
