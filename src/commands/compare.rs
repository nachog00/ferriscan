use anyhow::Result;
use clap::Args;
use colored::Colorize;
use std::path::PathBuf;

use super::SubCommand;
use crate::report::OutputFormat;
use crate::{analyzer, compare, workspace};

#[derive(Args)]
pub struct CompareCommand {
    /// Path to the first (left) workspace
    left: PathBuf,

    /// Path to the second (right) workspace
    right: PathBuf,

    /// Output format
    #[arg(short, long, default_value = "table")]
    format: OutputFormat,
}

impl SubCommand for CompareCommand {
    fn run(self) -> Result<()> {
        let left = self.left.canonicalize().unwrap_or(self.left);
        let right = self.right.canonicalize().unwrap_or(self.right);

        let left_name = left
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "left".to_string());
        let right_name = right
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "right".to_string());

        eprintln!(
            "{} {} vs {}",
            "Comparing:".cyan().bold(),
            left_name,
            right_name
        );

        let left_crates = workspace::discover_crates(&left)?;
        let right_crates = workspace::discover_crates(&right)?;

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
