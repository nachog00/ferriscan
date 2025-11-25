use std::fmt;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

/// A resolved repository ready for analysis.
/// Holds a temp directory if cloned, ensuring cleanup on drop.
pub struct ResolvedRepo {
    path: PathBuf,
    _temp_dir: Option<TempDir>,
}

impl ResolvedRepo {
    /// Create from a local path (no temp directory).
    pub fn from_local(path: PathBuf) -> Self {
        Self {
            path,
            _temp_dir: None,
        }
    }

    /// Create from a cloned temp directory.
    pub fn from_temp_dir(temp_dir: TempDir) -> Self {
        Self {
            path: temp_dir.path().to_path_buf(),
            _temp_dir: Some(temp_dir),
        }
    }

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
