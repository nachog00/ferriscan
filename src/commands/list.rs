use anyhow::Result;
use clap::Args;
use colored::Colorize;

use super::SubCommand;
use crate::primitives::resolved_path::ResolvedPath;
use crate::workspace;

#[derive(Args)]
pub struct ListCommand {
    /// Path to the workspace
    #[arg(default_value = ".")]
    path: ResolvedPath,
}

impl SubCommand for ListCommand {
    fn run(self) -> Result<()> {
        let crates = workspace::discover_crates(&self.path)?;

        println!("{}", "Discovered crates:".cyan().bold());
        for crate_info in &crates {
            println!("  {} - {}", crate_info.name, crate_info.path.display());
        }
        println!("\n{} crates total", crates.len());

        Ok(())
    }
}
