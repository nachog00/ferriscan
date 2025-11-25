use crate::analyzer::ItemCounts;
use crate::primitives::file_count::FileCount;
use clap::ValueEnum;
use comfy_table::{presets::UTF8_FULL, Cell, Color, ContentArrangement, Table};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Metrics for a single file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetrics {
    pub path: String,
    pub sloc: f64,
    pub ploc: f64,
    pub lloc: f64,
    pub cloc: f64,
    pub blank: f64,
    pub functions: f64,
    pub closures: f64,
    pub cyclomatic_sum: f64,
    pub cyclomatic_avg: f64,
    pub cognitive_sum: f64,
    pub cognitive_avg: f64,
    pub halstead_difficulty: f64,
    pub halstead_effort: f64,
    pub halstead_bugs: f64,
    pub mi_visual_studio: f64,
    pub mi_sei: f64,
    pub mi_original: f64,
    pub nargs_total: f64,
    pub nargs_avg: f64,
    pub nexits_sum: f64,
    #[serde(skip)]
    pub item_counts: ItemCounts,
}

/// Aggregated metrics for a crate.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CrateReport {
    pub name: String,
    #[serde(skip_serializing_if = "is_default_path")]
    pub path: PathBuf,
    pub file_count: FileCount,
    pub total_sloc: f64,
    pub total_ploc: f64,
    pub total_lloc: f64,
    pub total_cloc: f64,
    pub total_blank: f64,
    pub total_functions: f64,
    pub total_closures: f64,
    pub avg_file_sloc: f64,
    pub avg_functions_per_file: f64,
    pub avg_cyclomatic: f64,
    pub avg_cognitive: f64,
    pub avg_mi: f64,
    pub avg_halstead_difficulty: f64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<FileMetrics>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub largest_files: Vec<FileMetrics>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub most_complex_files: Vec<FileMetrics>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub worst_maintainability: Vec<FileMetrics>,
}

fn is_default_path(p: &PathBuf) -> bool {
    p.as_os_str().is_empty()
}

/// Aggregated metrics for an entire workspace.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkspaceReport {
    pub total_crates: usize,
    pub total_files: FileCount,
    pub total_sloc: f64,
    pub total_functions: f64,
    pub avg_cyclomatic: f64,
    pub avg_cognitive: f64,
    pub avg_mi: f64,
    pub crates: Vec<CrateReport>,
}

/// Comparison between two workspace reports.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonReport {
    pub left_name: String,
    pub right_name: String,
    pub left: WorkspaceSummary,
    pub right: WorkspaceSummary,
    pub delta: MetricsDelta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSummary {
    pub total_crates: usize,
    pub total_files: FileCount,
    pub total_sloc: f64,
    pub total_functions: f64,
    pub avg_cyclomatic: f64,
    pub avg_cognitive: f64,
    pub avg_mi: f64,
    pub sloc_per_file: f64,
    pub functions_per_file: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsDelta {
    pub sloc_diff: f64,
    pub sloc_pct: f64,
    pub files_diff: i64,
    pub functions_diff: f64,
    pub cyclomatic_diff: f64,
    pub cognitive_diff: f64,
    pub mi_diff: f64,
    pub sloc_per_file_diff: f64,
    pub functions_per_file_diff: f64,
}

impl WorkspaceReport {
    pub fn to_summary(&self) -> WorkspaceSummary {
        WorkspaceSummary {
            total_crates: self.total_crates,
            total_files: self.total_files,
            total_sloc: self.total_sloc,
            total_functions: self.total_functions,
            avg_cyclomatic: self.avg_cyclomatic,
            avg_cognitive: self.avg_cognitive,
            avg_mi: self.avg_mi,
            sloc_per_file: self.total_sloc / self.total_files,
            functions_per_file: self.total_functions / self.total_files,
        }
    }
}

/// Thresholds for metric warnings.
#[derive(Debug, Clone)]
pub struct Thresholds {
    pub max_file_sloc: f64,
    pub max_cyclomatic: f64,
    pub max_cognitive: f64,
    pub min_mi: f64,
    pub max_functions_per_file: f64,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            max_file_sloc: 500.0,
            max_cyclomatic: 10.0,
            max_cognitive: 15.0,
            min_mi: 20.0,
            max_functions_per_file: 20.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warning {
    pub file: String,
    pub metric: String,
    pub value: f64,
    pub threshold: f64,
    pub message: String,
}

impl CrateReport {
    pub fn check_thresholds(&self, thresholds: &Thresholds) -> Vec<Warning> {
        let mut warnings = Vec::new();

        for file in &self.files {
            if file.sloc > thresholds.max_file_sloc {
                warnings.push(Warning {
                    file: file.path.clone(),
                    metric: "sloc".to_string(),
                    value: file.sloc,
                    threshold: thresholds.max_file_sloc,
                    message: format!(
                        "File has {} SLOC (threshold: {})",
                        file.sloc as i64, thresholds.max_file_sloc as i64
                    ),
                });
            }

            if file.cyclomatic_avg > thresholds.max_cyclomatic {
                warnings.push(Warning {
                    file: file.path.clone(),
                    metric: "cyclomatic".to_string(),
                    value: file.cyclomatic_avg,
                    threshold: thresholds.max_cyclomatic,
                    message: format!(
                        "High cyclomatic complexity: {:.1} (threshold: {:.1})",
                        file.cyclomatic_avg, thresholds.max_cyclomatic
                    ),
                });
            }

            if file.cognitive_avg > thresholds.max_cognitive {
                warnings.push(Warning {
                    file: file.path.clone(),
                    metric: "cognitive".to_string(),
                    value: file.cognitive_avg,
                    threshold: thresholds.max_cognitive,
                    message: format!(
                        "High cognitive complexity: {:.1} (threshold: {:.1})",
                        file.cognitive_avg, thresholds.max_cognitive
                    ),
                });
            }

            if file.mi_visual_studio < thresholds.min_mi && file.mi_visual_studio > 0.0 {
                warnings.push(Warning {
                    file: file.path.clone(),
                    metric: "maintainability".to_string(),
                    value: file.mi_visual_studio,
                    threshold: thresholds.min_mi,
                    message: format!(
                        "Low maintainability index: {:.1} (threshold: {:.1})",
                        file.mi_visual_studio, thresholds.min_mi
                    ),
                });
            }

            if file.functions > thresholds.max_functions_per_file {
                warnings.push(Warning {
                    file: file.path.clone(),
                    metric: "functions".to_string(),
                    value: file.functions,
                    threshold: thresholds.max_functions_per_file,
                    message: format!(
                        "Too many functions: {} (threshold: {})",
                        file.functions as i64, thresholds.max_functions_per_file as i64
                    ),
                });
            }
        }

        warnings
    }
}

// Output formatting

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
    JsonPretty,
}

impl WorkspaceReport {
    pub fn format(&self, format: &OutputFormat, verbose: bool) -> String {
        match format {
            OutputFormat::Json => serde_json::to_string(self).unwrap_or_default(),
            OutputFormat::JsonPretty => serde_json::to_string_pretty(self).unwrap_or_default(),
            OutputFormat::Table => self.format_table(verbose),
        }
    }

    fn format_table(&self, verbose: bool) -> String {
        let mut output = String::new();

        // Summary table
        output.push_str("\n=== Workspace Summary ===\n\n");

        let mut summary = Table::new();
        summary.load_preset(UTF8_FULL);
        summary.set_content_arrangement(ContentArrangement::Dynamic);
        summary.set_header(vec!["Metric", "Value"]);
        summary.add_row(vec!["Total Crates", &self.total_crates.to_string()]);
        summary.add_row(vec!["Total Files", &self.total_files.to_string()]);
        summary.add_row(vec!["Total SLOC", &format!("{:.0}", self.total_sloc)]);
        summary.add_row(vec![
            "Total Functions",
            &format!("{:.0}", self.total_functions),
        ]);
        summary.add_row(vec![
            "Avg Cyclomatic",
            &format!("{:.2}", self.avg_cyclomatic),
        ]);
        summary.add_row(vec!["Avg Cognitive", &format!("{:.2}", self.avg_cognitive)]);
        summary.add_row(vec!["Avg Maintainability", &format!("{:.1}", self.avg_mi)]);

        if !self.total_files.is_zero() {
            summary.add_row(vec![
                "SLOC per File",
                &format!("{:.1}", self.total_sloc / self.total_files),
            ]);
            summary.add_row(vec![
                "Functions per File",
                &format!("{:.1}", self.total_functions / self.total_files),
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

        for crate_report in &self.crates {
            let mi_color = if crate_report.avg_mi < 20.0 {
                Color::Red
            } else if crate_report.avg_mi < 40.0 {
                Color::Yellow
            } else {
                Color::Green
            };

            crates_table.add_row(vec![
                Cell::new(&crate_report.name),
                Cell::new(crate_report.file_count),
                Cell::new(format!("{:.0}", crate_report.total_sloc)),
                Cell::new(format!("{:.0}", crate_report.total_functions)),
                Cell::new(format!("{:.1}", crate_report.avg_cyclomatic)),
                Cell::new(format!("{:.1}", crate_report.avg_cognitive)),
                Cell::new(format!("{:.1}", crate_report.avg_mi)).fg(mi_color),
            ]);
        }

        output.push_str(&crates_table.to_string());

        if verbose {
            // Show top files across workspace
            output.push_str("\n\n=== Largest Files (Top 10) ===\n\n");

            let mut all_files: Vec<_> = self
                .crates
                .iter()
                .flat_map(|c| c.largest_files.iter().map(|f| (&c.name, f)))
                .collect();
            all_files.sort_by(|a, b| b.1.sloc.partial_cmp(&a.1.sloc).unwrap());
            all_files.truncate(10);

            let mut files_table = Table::new();
            files_table.load_preset(UTF8_FULL);
            files_table.set_content_arrangement(ContentArrangement::Dynamic);
            files_table.set_header(vec!["Crate", "File", "SLOC", "Funcs", "Cyc", "MI"]);

            for (crate_name, file) in all_files {
                files_table.add_row(vec![
                    crate_name,
                    &file.path,
                    &format!("{:.0}", file.sloc),
                    &format!("{:.0}", file.functions),
                    &format!("{:.1}", file.cyclomatic_avg),
                    &format!("{:.1}", file.mi_visual_studio),
                ]);
            }

            output.push_str(&files_table.to_string());

            // Show worst maintainability
            output.push_str("\n\n=== Worst Maintainability (Top 10) ===\n\n");

            let mut worst: Vec<_> = self
                .crates
                .iter()
                .flat_map(|c| c.worst_maintainability.iter().map(|f| (&c.name, f)))
                .filter(|(_, f)| f.mi_visual_studio > 0.0)
                .collect();
            worst.sort_by(|a, b| {
                a.1.mi_visual_studio
                    .partial_cmp(&b.1.mi_visual_studio)
                    .unwrap()
            });
            worst.truncate(10);

            let mut worst_table = Table::new();
            worst_table.load_preset(UTF8_FULL);
            worst_table.set_content_arrangement(ContentArrangement::Dynamic);
            worst_table.set_header(vec!["Crate", "File", "MI", "SLOC", "Cyc"]);

            for (crate_name, file) in worst {
                let mi_color = if file.mi_visual_studio < 20.0 {
                    Color::Red
                } else if file.mi_visual_studio < 40.0 {
                    Color::Yellow
                } else {
                    Color::Green
                };

                worst_table.add_row(vec![
                    Cell::new(crate_name),
                    Cell::new(&file.path),
                    Cell::new(format!("{:.1}", file.mi_visual_studio)).fg(mi_color),
                    Cell::new(format!("{:.0}", file.sloc)),
                    Cell::new(format!("{:.1}", file.cyclomatic_avg)),
                ]);
            }

            output.push_str(&worst_table.to_string());
        }

        output
    }
}

impl ComparisonReport {
    pub fn format(&self, format: &OutputFormat) -> String {
        match format {
            OutputFormat::Json => serde_json::to_string(self).unwrap_or_default(),
            OutputFormat::JsonPretty => serde_json::to_string_pretty(self).unwrap_or_default(),
            OutputFormat::Table => self.format_table(),
        }
    }

    fn format_table(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            "\n=== Comparison: {} vs {} ===\n\n",
            self.left_name, self.right_name
        ));

        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        table.set_content_arrangement(ContentArrangement::Dynamic);
        table.set_header(vec![
            "Metric",
            &self.left_name,
            &self.right_name,
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
            Cell::new(self.left.total_crates),
            Cell::new(self.right.total_crates),
            Cell::new(""),
            Cell::new(""),
        ]);

        table.add_row(vec![
            Cell::new("Files"),
            Cell::new(self.left.total_files),
            Cell::new(self.right.total_files),
            Cell::new(format!("{:+}", self.delta.files_diff)),
            Cell::new(""),
        ]);

        table.add_row(vec![
            Cell::new("SLOC"),
            Cell::new(format!("{:.0}", self.left.total_sloc)),
            Cell::new(format!("{:.0}", self.right.total_sloc)),
            delta_cell(self.delta.sloc_diff, false),
            pct_cell(self.delta.sloc_pct, false),
        ]);

        table.add_row(vec![
            Cell::new("Functions"),
            Cell::new(format!("{:.0}", self.left.total_functions)),
            Cell::new(format!("{:.0}", self.right.total_functions)),
            delta_cell(self.delta.functions_diff, false),
            Cell::new(""),
        ]);

        table.add_row(vec![
            Cell::new("SLOC/File"),
            Cell::new(format!("{:.1}", self.left.sloc_per_file)),
            Cell::new(format!("{:.1}", self.right.sloc_per_file)),
            delta_cell(self.delta.sloc_per_file_diff, false),
            Cell::new(""),
        ]);

        table.add_row(vec![
            Cell::new("Funcs/File"),
            Cell::new(format!("{:.1}", self.left.functions_per_file)),
            Cell::new(format!("{:.1}", self.right.functions_per_file)),
            delta_cell(self.delta.functions_per_file_diff, false),
            Cell::new(""),
        ]);

        table.add_row(vec![
            Cell::new("Avg Cyclomatic"),
            Cell::new(format!("{:.2}", self.left.avg_cyclomatic)),
            Cell::new(format!("{:.2}", self.right.avg_cyclomatic)),
            delta_cell(self.delta.cyclomatic_diff, false),
            Cell::new(""),
        ]);

        table.add_row(vec![
            Cell::new("Avg Cognitive"),
            Cell::new(format!("{:.2}", self.left.avg_cognitive)),
            Cell::new(format!("{:.2}", self.right.avg_cognitive)),
            delta_cell(self.delta.cognitive_diff, false),
            Cell::new(""),
        ]);

        table.add_row(vec![
            Cell::new("Avg MI"),
            Cell::new(format!("{:.1}", self.left.avg_mi)),
            Cell::new(format!("{:.1}", self.right.avg_mi)),
            delta_cell(self.delta.mi_diff, true), // higher MI is better
            Cell::new(""),
        ]);

        output.push_str(&table.to_string());
        output
    }
}
