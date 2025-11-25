use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Args;
use colored::Colorize;

use crate::cli::args::RepoSource;
use crate::cli::SubCommand;
use ferriscan::report::OutputFormat;
use ferriscan::{analyzer, compare, workspace};

#[derive(Args)]
pub struct CompareCommand {
    /// First source to compare (local path or git URL with optional @ref)
    left_source: RepoSource,

    /// Second source to compare (local path or git URL with optional @ref)
    right_source: RepoSource,

    /// Relative path within the first source (also used for second if not specified)
    #[arg(default_value = ".")]
    left_path: PathBuf,

    /// Relative path within the second source (defaults to left_path)
    right_path: Option<PathBuf>,

    /// Output format
    #[arg(short, long, default_value = "table")]
    format: OutputFormat,
}

impl SubCommand for CompareCommand {
    fn run(self) -> Result<()> {
        let left_name = self.left_source.display_name();
        let right_name = self.right_source.display_name();

        // Resolve left source
        if self.left_source.is_remote() {
            eprintln!("{} {}...", "Cloning:".cyan().bold(), self.left_source);
        }
        let left_resolved = self
            .left_source
            .resolve()
            .context("failed to resolve left source")?;

        // Resolve right source
        if self.right_source.is_remote() {
            eprintln!("{} {}...", "Cloning:".cyan().bold(), self.right_source);
        }
        let right_resolved = self
            .right_source
            .resolve()
            .context("failed to resolve right source")?;

        // Use left_path for right if right_path not specified
        let right_path = self.right_path.unwrap_or_else(|| self.left_path.clone());

        let left_target = left_resolved.path().join(&self.left_path);
        let right_target = right_resolved.path().join(&right_path);

        eprintln!(
            "{} {} vs {}",
            "Comparing:".cyan().bold(),
            left_name,
            right_name
        );

        let left_crates = workspace::discover_crates(&left_target)?;
        let right_crates = workspace::discover_crates(&right_target)?;

        eprintln!(
            "  {} {} crates, {} {} crates",
            left_name,
            left_crates.len(),
            right_name,
            right_crates.len()
        );

        let left_report = analyzer::analyze_workspace(&left_crates);
        let right_report = analyzer::analyze_workspace(&right_crates);

        let comparison =
            compare::compare_workspaces(&left_name, &left_report, &right_name, &right_report);

        println!("{}", comparison.format(&self.format));

        Ok(())
    }
}
