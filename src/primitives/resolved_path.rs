use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// A path that has been resolved to its canonical form if possible.
///
/// On construction, attempts `canonicalize()` to resolve symlinks and
/// normalize the path. Falls back to the original if resolution fails
/// (e.g., path doesn't exist yet).
#[derive(Clone, Debug)]
pub struct ResolvedPath(PathBuf);

impl ResolvedPath {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        Self(path.canonicalize().unwrap_or(path))
    }
}

impl AsRef<Path> for ResolvedPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl std::ops::Deref for ResolvedPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for ResolvedPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.display().fmt(f)
    }
}

impl FromStr for ResolvedPath {
    type Err = <PathBuf as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(PathBuf::from_str(s)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn new_with_existing_path_canonicalizes() {
        let cwd = env::current_dir().unwrap();
        let resolved = ResolvedPath::new(".");

        assert!(resolved.0.is_absolute());
        assert_eq!(resolved.0, cwd);
    }

    #[test]
    fn new_with_nonexistent_path_falls_back() {
        let resolved = ResolvedPath::new("/nonexistent/path/that/does/not/exist");

        assert_eq!(resolved.0, PathBuf::from("/nonexistent/path/that/does/not/exist"));
    }

    #[test]
    fn from_str_works() {
        let resolved: ResolvedPath = ".".parse().unwrap();
        let cwd = env::current_dir().unwrap();

        assert_eq!(resolved.0, cwd);
    }

    #[test]
    fn display_shows_path() {
        let resolved = ResolvedPath::new("/some/path");
        assert_eq!(format!("{}", resolved), "/some/path");
    }

    #[test]
    fn deref_to_path() {
        let resolved = ResolvedPath::new("/some/path");
        let path: &Path = &resolved;

        assert_eq!(path, Path::new("/some/path"));
    }

    #[test]
    fn as_ref_path() {
        let resolved = ResolvedPath::new("/some/path");
        let path: &Path = resolved.as_ref();

        assert_eq!(path, Path::new("/some/path"));
    }
}
