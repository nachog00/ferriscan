use std::path::PathBuf;

use anyhow::Result;
use clap::Args;
use colored::Colorize;

use crate::cli::format::{format_workspace, OutputFormat};
use crate::cli::repo::source::RepoSource;
use crate::cli::SubCommand;
use ferriscan::domain::analysis::analyze_workspace;
use ferriscan::domain::thresholds::{check_crate_thresholds, Thresholds, Warning};
use ferriscan::extraction::rca::RcaExtractor;
use ferriscan::workspace;

#[derive(Args)]
pub struct AnalyzeCommand {
    /// Source to analyze (local path or git URL with optional @ref)
    #[arg(default_value = ".")]
    source: RepoSource,

    /// Relative path within the source to analyze
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Output format
    #[arg(short, long, default_value = "table")]
    format: OutputFormat,

    /// Show verbose output (detailed file listings)
    #[arg(short, long)]
    verbose: bool,

    /// Check thresholds and report warnings
    #[arg(short = 'w', long)]
    warnings: bool,

    /// Maximum SLOC per file threshold
    #[arg(long, default_value = "500")]
    max_sloc: f64,

    /// Maximum cyclomatic complexity threshold
    #[arg(long, default_value = "10")]
    max_cyclomatic: f64,

    /// Maximum cognitive complexity threshold
    #[arg(long, default_value = "15")]
    max_cognitive: f64,

    /// Minimum maintainability index threshold
    #[arg(long, default_value = "20")]
    min_mi: f64,
}

impl SubCommand for AnalyzeCommand {
    fn run(self) -> Result<()> {
        let source_name = self.source.display_name();
        let resolved = self.source.resolve_with_progress()?;
        let target_path = resolved.path().join(&self.path);

        eprintln!("{} {}", "Analyzing:".cyan().bold(), source_name);

        let crates = workspace::discover_crates(&target_path)?;
        eprintln!("{} {} crates", "Found:".cyan().bold(), crates.len());

        let extractor = RcaExtractor;
        let workspace_metrics = analyze_workspace(&extractor, &crates);

        println!("{}", format_workspace(&workspace_metrics, &self.format, self.verbose));

        if self.warnings {
            let thresholds = self.thresholds();
            let warnings: Vec<_> = workspace_metrics
                .crates
                .iter()
                .flat_map(|c| {
                    check_crate_thresholds(c, &thresholds)
                        .into_iter()
                        .map(|w| (c.name.clone(), w))
                })
                .collect();
            print_warnings(&warnings);
        }

        Ok(())
    }
}

impl AnalyzeCommand {
    fn thresholds(&self) -> Thresholds {
        Thresholds {
            max_file_sloc: self.max_sloc,
            max_cyclomatic: self.max_cyclomatic,
            max_cognitive: self.max_cognitive,
            min_mi: self.min_mi,
            ..Default::default()
        }
    }
}

fn print_warnings(warnings: &[(String, Warning)]) {
    if warnings.is_empty() {
        eprintln!("\n{}", "No warnings".green().bold());
        return;
    }

    eprintln!("\n{}", "=== Warnings ===".yellow().bold());
    for (crate_name, warning) in warnings {
        let location = match (&warning.function, warning.line) {
            (Some(func), Some(line)) => format!("{}:{} ({})", warning.file, line, func),
            (Some(func), None) => format!("{} ({})", warning.file, func),
            (None, Some(line)) => format!("{}:{}", warning.file, line),
            (None, None) => warning.file.clone(),
        };

        eprintln!(
            "  {} {}: {}",
            format!("[{}]", crate_name).dimmed(),
            location.yellow(),
            warning.message
        );
    }
    eprintln!("\n{} {} warnings", "Total:".yellow().bold(), warnings.len());
}
