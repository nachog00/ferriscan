mod cli;

use anyhow::Result;
use clap::{Parser, Subcommand};

use cli::commands::analyze::AnalyzeCommand;
use cli::commands::compare::CompareCommand;
use cli::commands::list::ListCommand;
use cli::SubCommand;

#[derive(Parser)]
#[command(name = "ferriscan")]
#[command(about = "Analyze and compare Rust crate/workspace code metrics")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a single workspace or crate
    Analyze(AnalyzeCommand),

    /// Compare two workspaces or crates
    Compare(CompareCommand),

    /// List discovered crates in a workspace
    List(ListCommand),
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Commands::Analyze(cmd) => cmd.run(),
        Commands::Compare(cmd) => cmd.run(),
        Commands::List(cmd) => cmd.run(),
    }
}
