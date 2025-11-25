use anyhow::Result;
use clap::Args;
use colored::Colorize;

use crate::cli::SubCommand;
use ferriscan::primitives::resolved_path::ResolvedPath;
use ferriscan::report::OutputFormat;
use ferriscan::{analyzer, compare, workspace};

#[derive(Args)]
pub struct CompareCommand {
    /// Path to the first (left) workspace
    left: ResolvedPath,

    /// Path to the second (right) workspace
    right: ResolvedPath,

    /// Output format
    #[arg(short, long, default_value = "table")]
    format: OutputFormat,
}

impl SubCommand for CompareCommand {
    fn run(self) -> Result<()> {
        let left_name = self
            .left
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "left".to_string());
        let right_name = self
            .right
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "right".to_string());

        eprintln!(
            "{} {} vs {}",
            "Comparing:".cyan().bold(),
            left_name,
            right_name
        );

        let left_crates = workspace::discover_crates(&self.left)?;
        let right_crates = workspace::discover_crates(&self.right)?;

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
