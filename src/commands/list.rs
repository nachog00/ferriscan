use anyhow::Result;
use clap::Args;
use colored::Colorize;
use std::path::PathBuf;

use super::SubCommand;
use crate::workspace;

#[derive(Args)]
pub struct ListCommand {
    /// Path to the workspace
    #[arg(default_value = ".")]
    path: PathBuf,
}

impl SubCommand for ListCommand {
    fn run(self) -> Result<()> {
        let path = self.path.canonicalize().unwrap_or(self.path);
        let crates = workspace::discover_crates(&path)?;

        println!("{}", "Discovered crates:".cyan().bold());
        for crate_info in &crates {
            println!("  {} - {}", crate_info.name, crate_info.path.display());
        }
        println!("\n{} crates total", crates.len());

        Ok(())
    }
}
