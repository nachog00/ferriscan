use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

use super::remote::{GitRef, RemoteRepo};
use super::resolved::{RepoSourceError, ResolvedRepo};

/// A source of code to analyze - either a local path or a remote git URL.
///
/// Supports `@ref` suffix for specifying branches/tags on remote URLs:
/// - `https://github.com/user/repo@main`
/// - `git@github.com:user/repo@v1.0.0`
#[derive(Debug, Clone)]
pub enum RepoSource {
    Local(PathBuf),
    Remote(RemoteRepo),
}

impl RepoSource {
    /// Resolve the source to a local path, cloning if necessary.
    pub fn resolve(&self) -> Result<ResolvedRepo, RepoSourceError> {
        match self {
            Self::Local(path) => {
                let resolved = path.canonicalize().unwrap_or_else(|_| path.clone());
                Ok(ResolvedRepo::from_local(resolved))
            }
            Self::Remote(remote) => remote.clone_repo(),
        }
    }

    /// Resolve with a progress message to stderr for remote sources.
    pub fn resolve_with_progress(&self) -> Result<ResolvedRepo, RepoSourceError> {
        use colored::Colorize;

        if self.is_remote() {
            eprintln!("{} {}...", "Cloning:".cyan().bold(), self);
        }
        self.resolve()
    }

    pub fn is_remote(&self) -> bool {
        matches!(self, Self::Remote(_))
    }

    /// Get a display name for this source (repo name, optionally with ref).
    pub fn display_name(&self) -> String {
        match self {
            Self::Local(path) => path
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| path.display().to_string()),
            Self::Remote(remote) => remote.display_name(),
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
            let git_ref = match git_ref {
                Some(r) => GitRef::Named(r.to_string()),
                None => GitRef::Default,
            };
            Ok(Self::Remote(RemoteRepo::new(url.to_string(), git_ref)))
        } else {
            Ok(Self::Local(PathBuf::from(s)))
        }
    }
}

impl fmt::Display for RepoSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Local(path) => write!(f, "{}", path.display()),
            Self::Remote(remote) => match &remote.git_ref {
                GitRef::Named(r) => write!(f, "{}@{}", remote.url, r),
                GitRef::Default => write!(f, "{}", remote.url),
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
            RepoSource::Remote(RemoteRepo { git_ref: GitRef::Default, .. })
        ));
    }

    #[test]
    fn git_ssh_url_detected() {
        let source: RepoSource = "git@github.com:user/repo".parse().unwrap();
        assert!(matches!(
            source,
            RepoSource::Remote(RemoteRepo { git_ref: GitRef::Default, .. })
        ));
    }

    #[test]
    fn https_with_ref_parsed() {
        let source: RepoSource = "https://github.com/user/repo@main".parse().unwrap();
        assert!(matches!(
            source,
            RepoSource::Remote(RemoteRepo { git_ref: GitRef::Named(ref r), .. }) if r == "main"
        ));
    }

    #[test]
    fn git_ssh_with_ref_parsed() {
        let source: RepoSource = "git@github.com:user/repo@v1.0.0".parse().unwrap();
        assert!(matches!(
            source,
            RepoSource::Remote(RemoteRepo { git_ref: GitRef::Named(ref r), .. }) if r == "v1.0.0"
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
