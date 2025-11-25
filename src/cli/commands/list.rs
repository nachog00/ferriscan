use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Args;
use colored::Colorize;

use crate::cli::repo::source::RepoSource;
use crate::cli::SubCommand;
use ferriscan::workspace;

#[derive(Args)]
pub struct ListCommand {
    /// Source to list (local path or git URL with optional @ref)
    #[arg(default_value = ".")]
    source: RepoSource,

    /// Relative path within the source
    #[arg(default_value = ".")]
    path: PathBuf,
}

impl SubCommand for ListCommand {
    fn run(self) -> Result<()> {
        if self.source.is_remote() {
            eprintln!("{} {}...", "Cloning:".cyan().bold(), self.source);
        }

        let resolved = self
            .source
            .resolve()
            .context("failed to resolve source")?;

        let target_path = resolved.path().join(&self.path);
        let crates = workspace::discover_crates(&target_path)?;

        println!("{}", "Discovered crates:".cyan().bold());
        for crate_info in &crates {
            println!("  {} - {}", crate_info.name, crate_info.path.display());
        }
        println!("\n{} crates total", crates.len());

        Ok(())
    }
}
