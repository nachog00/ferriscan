use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Information about a crate.
#[derive(Debug, Clone)]
pub struct CrateInfo {
    pub name: String,
    pub path: PathBuf,
}

/// Workspace Cargo.toml structure (partial).
#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Option<Package>,
    workspace: Option<Workspace>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Workspace {
    members: Option<Vec<String>>,
}

/// Discover crates in a workspace or single crate.
pub fn discover_crates(path: &Path) -> Result<Vec<CrateInfo>> {
    let cargo_toml = path.join("Cargo.toml");

    if !cargo_toml.exists() {
        anyhow::bail!("No Cargo.toml found at {}", path.display());
    }

    let content = std::fs::read_to_string(&cargo_toml)
        .with_context(|| format!("Failed to read {}", cargo_toml.display()))?;

    let parsed: CargoToml = toml::from_str(&content)
        .with_context(|| format!("Failed to parse {}", cargo_toml.display()))?;

    let mut crates = Vec::new();

    // Check if it's a workspace
    if let Some(workspace) = parsed.workspace {
        if let Some(members) = workspace.members {
            for member in members {
                // Handle glob patterns (simple version)
                if member.contains('*') {
                    let pattern = path.join(&member);
                    for entry in glob::glob(pattern.to_str().unwrap_or("")).into_iter().flatten() {
                        if let Ok(member_path) = entry {
                            if member_path.is_dir() && member_path.join("Cargo.toml").exists() {
                                if let Some(info) = parse_crate_info(&member_path) {
                                    crates.push(info);
                                }
                            }
                        }
                    }
                } else {
                    let member_path = path.join(&member);
                    if member_path.is_dir() && member_path.join("Cargo.toml").exists() {
                        if let Some(info) = parse_crate_info(&member_path) {
                            crates.push(info);
                        }
                    }
                }
            }
        }
    }

    // If no workspace members found, treat as single crate
    if crates.is_empty() {
        if let Some(info) = parse_crate_info(path) {
            crates.push(info);
        }
    }

    Ok(crates)
}

/// Parse crate info from a directory containing Cargo.toml.
fn parse_crate_info(path: &Path) -> Option<CrateInfo> {
    let cargo_toml = path.join("Cargo.toml");
    let content = std::fs::read_to_string(&cargo_toml).ok()?;
    let parsed: CargoToml = toml::from_str(&content).ok()?;

    let name = parsed
        .package
        .map(|p| p.name)
        .or_else(|| {
            path.file_name()
                .map(|s| s.to_string_lossy().to_string())
        })?;

    Some(CrateInfo {
        name,
        path: path.to_path_buf(),
    })
}

/// Discover crates from multiple paths (for comparison).
pub fn discover_all(paths: &[PathBuf]) -> Result<Vec<(String, Vec<CrateInfo>)>> {
    let mut result = Vec::new();

    for path in paths {
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| path.display().to_string());

        let crates = discover_crates(path)?;
        result.push((name, crates));
    }

    Ok(result)
}
