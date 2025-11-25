use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Args;
use colored::Colorize;

use crate::cli::repo::source::RepoSource;
use crate::cli::SubCommand;
use ferriscan::report::{OutputFormat, Thresholds};
use ferriscan::{analyzer, workspace};

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

        if self.source.is_remote() {
            eprintln!("{} {}...", "Cloning:".cyan().bold(), self.source);
        }

        let resolved = self
            .source
            .resolve()
            .context("failed to resolve source")?;

        let target_path = resolved.path().join(&self.path);

        eprintln!("{} {}", "Analyzing:".cyan().bold(), source_name);

        let crates = workspace::discover_crates(&target_path)?;
        eprintln!("{} {} crates", "Found:".cyan().bold(), crates.len());

        let report = analyzer::analyze_workspace(&crates);
        println!("{}", report.format(&self.format, self.verbose));

        if self.warnings {
            let thresholds = Thresholds {
                max_file_sloc: self.max_sloc,
                max_cyclomatic: self.max_cyclomatic,
                max_cognitive: self.max_cognitive,
                min_mi: self.min_mi,
                ..Default::default()
            };

            let mut all_warnings = Vec::new();
            for crate_report in &report.crates {
                let crate_warnings = crate_report.check_thresholds(&thresholds);
                for warning in crate_warnings {
                    all_warnings.push((crate_report.name.clone(), warning));
                }
            }

            if !all_warnings.is_empty() {
                eprintln!("\n{}", "=== Warnings ===".yellow().bold());
                for (crate_name, warning) in &all_warnings {
                    eprintln!(
                        "  {} {}: {}",
                        format!("[{}]", crate_name).dimmed(),
                        warning.file.yellow(),
                        warning.message
                    );
                }
                eprintln!(
                    "\n{} {} warnings",
                    "Total:".yellow().bold(),
                    all_warnings.len()
                );
            } else {
                eprintln!("\n{}", "No warnings".green().bold());
            }
        }

        Ok(())
    }
}
