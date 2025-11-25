use std::process::Command;

use tempfile::TempDir;

use super::resolved::{RepoSourceError, ResolvedRepo};

/// Git reference for remote repositories.
#[derive(Debug, Clone)]
pub enum GitRef {
    /// Clone the default branch (no --branch flag).
    Default,
    /// Clone a specific branch or tag.
    Named(String),
}

/// A remote git repository that can be cloned.
#[derive(Debug, Clone)]
pub struct RemoteRepo {
    pub url: String,
    pub git_ref: GitRef,
}

impl RemoteRepo {
    pub fn new(url: String, git_ref: GitRef) -> Self {
        Self { url, git_ref }
    }

    /// Clone this repository to a temporary directory.
    pub fn clone_repo(&self) -> Result<ResolvedRepo, RepoSourceError> {
        let temp_dir = TempDir::new().map_err(|e| RepoSourceError::TempDir(e.to_string()))?;

        let mut cmd = Command::new("git");
        cmd.arg("clone").arg("--depth=1");

        if let GitRef::Named(ref r) = self.git_ref {
            cmd.arg("--branch").arg(r);
        }

        cmd.arg(&self.url).arg(temp_dir.path());

        let output = cmd
            .output()
            .map_err(|e| RepoSourceError::GitNotFound(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(RepoSourceError::CloneFailed(stderr.to_string()));
        }

        Ok(ResolvedRepo::from_temp_dir(temp_dir))
    }

    /// Extract the repository name from the URL.
    pub fn repo_name(&self) -> &str {
        self.url
            .trim_end_matches(".git")
            .rsplit('/')
            .next()
            .unwrap_or(&self.url)
    }

    /// Get a display name (repo name with optional ref).
    pub fn display_name(&self) -> String {
        match &self.git_ref {
            GitRef::Named(r) => format!("{}@{}", self.repo_name(), r),
            GitRef::Default => self.repo_name().to_string(),
        }
    }
}
