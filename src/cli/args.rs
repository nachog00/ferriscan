use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

use tempfile::TempDir;

/// A source of code to analyze - either a local path or a remote git URL.
///
/// Supports `@ref` suffix for specifying branches/tags on remote URLs:
/// - `https://github.com/user/repo@main`
/// - `git@github.com:user/repo@v1.0.0`
#[derive(Debug, Clone)]
pub enum RepoSource {
    Local(PathBuf),
    Remote { url: String, git_ref: GitRef },
}

/// Git reference for remote repositories.
#[derive(Debug, Clone)]
pub enum GitRef {
    /// Clone the default branch (no --branch flag).
    Default,
    /// Clone a specific branch or tag.
    Named(String),
}

/// A resolved repository ready for analysis.
/// Holds a temp directory if cloned, ensuring cleanup on drop.
pub struct ResolvedRepo {
    path: PathBuf,
    _temp_dir: Option<TempDir>,
}

impl ResolvedRepo {
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl AsRef<Path> for ResolvedRepo {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

#[derive(Debug)]
pub enum RepoSourceError {
    TempDir(String),
    GitNotFound(String),
    CloneFailed(String),
}

impl fmt::Display for RepoSourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TempDir(e) => write!(f, "failed to create temp directory: {}", e),
            Self::GitNotFound(e) => write!(f, "git not found: {}", e),
            Self::CloneFailed(e) => write!(f, "git clone failed: {}", e),
        }
    }
}

impl std::error::Error for RepoSourceError {}

impl RepoSource {
    /// Resolve the source to a local path, cloning if necessary.
    pub fn resolve(&self) -> Result<ResolvedRepo, RepoSourceError> {
        match self {
            Self::Local(path) => {
                let resolved = path.canonicalize().unwrap_or_else(|_| path.clone());
                Ok(ResolvedRepo {
                    path: resolved,
                    _temp_dir: None,
                })
            }
            Self::Remote { url, git_ref } => {
                let temp_dir =
                    TempDir::new().map_err(|e| RepoSourceError::TempDir(e.to_string()))?;

                let mut cmd = Command::new("git");
                cmd.arg("clone").arg("--depth=1");

                if let GitRef::Named(r) = git_ref {
                    cmd.arg("--branch").arg(r);
                }

                cmd.arg(url).arg(temp_dir.path());

                let output = cmd
                    .output()
                    .map_err(|e| RepoSourceError::GitNotFound(e.to_string()))?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(RepoSourceError::CloneFailed(stderr.to_string()));
                }

                let path = temp_dir.path().to_path_buf();
                Ok(ResolvedRepo {
                    path,
                    _temp_dir: Some(temp_dir),
                })
            }
        }
    }

    pub fn is_remote(&self) -> bool {
        matches!(self, Self::Remote { .. })
    }

    /// Get a display name for this source (repo name, optionally with ref).
    pub fn display_name(&self) -> String {
        match self {
            Self::Local(path) => path
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| path.display().to_string()),
            Self::Remote { url, git_ref } => {
                let name = url
                    .trim_end_matches(".git")
                    .rsplit('/')
                    .next()
                    .unwrap_or(url);
                match git_ref {
                    GitRef::Named(r) => format!("{}@{}", name, r),
                    GitRef::Default => name.to_string(),
                }
            }
        }
    }
}

fn is_remote_url(s: &str) -> bool {
    s.starts_with("https://")
        || s.starts_with("http://")
        || s.starts_with("git@")
        || s.starts_with("ssh://")
        || s.starts_with("git://")
}

/// Extract @ref suffix from a remote URL.
/// Returns (url_without_ref, Some(ref)) or (original_url, None).
///
/// Only looks for @ after the last / to avoid matching git@github.com.
fn parse_remote_ref(url: &str) -> (&str, Option<&str>) {
    let last_slash = match url.rfind('/') {
        Some(pos) => pos,
        None => return (url, None),
    };

    let after_slash = &url[last_slash..];
    match after_slash.rfind('@') {
        Some(at_pos) => {
            let absolute_at = last_slash + at_pos;
            (&url[..absolute_at], Some(&url[absolute_at + 1..]))
        }
        None => (url, None),
    }
}

impl FromStr for RepoSource {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_remote_url(s) {
            let (url, git_ref) = parse_remote_ref(s);
            Ok(Self::Remote {
                url: url.to_string(),
                git_ref: match git_ref {
                    Some(r) => GitRef::Named(r.to_string()),
                    None => GitRef::Default,
                },
            })
        } else {
            Ok(Self::Local(PathBuf::from(s)))
        }
    }
}

impl fmt::Display for RepoSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Local(path) => write!(f, "{}", path.display()),
            Self::Remote { url, git_ref } => match git_ref {
                GitRef::Named(r) => write!(f, "{}@{}", url, r),
                GitRef::Default => write!(f, "{}", url),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_path_detected() {
        let source: RepoSource = "./some/path".parse().unwrap();
        assert!(!source.is_remote());
    }

    #[test]
    fn https_url_detected() {
        let source: RepoSource = "https://github.com/user/repo".parse().unwrap();
        assert!(matches!(
            source,
            RepoSource::Remote { git_ref: GitRef::Default, .. }
        ));
    }

    #[test]
    fn git_ssh_url_detected() {
        let source: RepoSource = "git@github.com:user/repo".parse().unwrap();
        assert!(matches!(
            source,
            RepoSource::Remote { git_ref: GitRef::Default, .. }
        ));
    }

    #[test]
    fn https_with_ref_parsed() {
        let source: RepoSource = "https://github.com/user/repo@main".parse().unwrap();
        assert!(matches!(
            source,
            RepoSource::Remote { git_ref: GitRef::Named(ref r), .. } if r == "main"
        ));
    }

    #[test]
    fn git_ssh_with_ref_parsed() {
        let source: RepoSource = "git@github.com:user/repo@v1.0.0".parse().unwrap();
        assert!(matches!(
            source,
            RepoSource::Remote { git_ref: GitRef::Named(ref r), .. } if r == "v1.0.0"
        ));
    }

    #[test]
    fn display_name_extracts_repo() {
        let source: RepoSource = "https://github.com/user/my-repo.git@v1.0".parse().unwrap();
        assert_eq!(source.display_name(), "my-repo@v1.0");
    }

    #[test]
    fn display_name_local() {
        let source: RepoSource = "/home/user/my-project".parse().unwrap();
        assert_eq!(source.display_name(), "my-project");
    }
}
