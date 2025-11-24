mod analyzer;
mod compare;
mod report;
mod workspace;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use report::{OutputFormat, Thresholds};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "crate-metrics")]
#[command(about = "Analyze and compare Rust crate/workspace code metrics")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a single workspace or crate
    Analyze {
        /// Path to the workspace or crate
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Output format
        #[arg(short, long, default_value = "table")]
        format: Format,

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
    },

    /// Compare two workspaces or crates
    Compare {
        /// Path to the first (left) workspace
        left: PathBuf,

        /// Path to the second (right) workspace
        right: PathBuf,

        /// Output format
        #[arg(short, long, default_value = "table")]
        format: Format,
    },

    /// List discovered crates in a workspace
    List {
        /// Path to the workspace
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

#[derive(Clone, ValueEnum)]
enum Format {
    Table,
    Json,
    JsonPretty,
}

impl From<&Format> for OutputFormat {
    fn from(f: &Format) -> Self {
        match f {
            Format::Table => OutputFormat::Table,
            Format::Json => OutputFormat::Json,
            Format::JsonPretty => OutputFormat::JsonPretty,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze {
            path,
            format,
            verbose,
            warnings,
            max_sloc,
            max_cyclomatic,
            max_cognitive,
            min_mi,
        } => {
            let path = path.canonicalize().unwrap_or(path);
            eprintln!(
                "{} {}",
                "Analyzing:".cyan().bold(),
                path.display()
            );

            let crates = workspace::discover_crates(&path)?;
            eprintln!(
                "{} {} crates",
                "Found:".cyan().bold(),
                crates.len()
            );

            let report = analyzer::analyze_workspace(&crates);
            println!("{}", report.format(&(&format).into(), verbose));

            if warnings {
                let thresholds = Thresholds {
                    max_file_sloc: max_sloc,
                    max_cyclomatic,
                    max_cognitive,
                    min_mi,
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
        }

        Commands::Compare {
            left,
            right,
            format,
        } => {
            let left = left.canonicalize().unwrap_or(left);
            let right = right.canonicalize().unwrap_or(right);

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

            println!("{}", comparison.format(&(&format).into()));
        }

        Commands::List { path } => {
            let path = path.canonicalize().unwrap_or(path);
            let crates = workspace::discover_crates(&path)?;

            println!("{}", "Discovered crates:".cyan().bold());
            for crate_info in &crates {
                println!("  {} - {}", crate_info.name, crate_info.path.display());
            }
            println!("\n{} crates total", crates.len());
        }
    }

    Ok(())
}
