use std::path::PathBuf;

use anyhow::Result;
use clap::Args;
use colored::Colorize;

use crate::cli::format::{compare_workspaces, format_comparison, OutputFormat};
use crate::cli::repo::source::RepoSource;
use crate::cli::SubCommand;
use ferriscan::domain::analysis::analyze_workspace;
use ferriscan::extraction::rca::RcaExtractor;
use ferriscan::workspace;

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

        let left_resolved = self.left_source.resolve_with_progress()?;
        let right_resolved = self.right_source.resolve_with_progress()?;

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

        let extractor = RcaExtractor;
        let left_metrics = analyze_workspace(&extractor, &left_crates);
        let right_metrics = analyze_workspace(&extractor, &right_crates);

        let comparison = compare_workspaces(left_name, right_name, &left_metrics, &right_metrics);

        println!("{}", format_comparison(&comparison, &self.format));

        Ok(())
    }
}
