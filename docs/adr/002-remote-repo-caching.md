# ADR 002: Remote Repository Caching

## Status
Proposed

## Context
Currently, every invocation of ferriscan with a remote URL clones the repository to a temporary directory that is deleted when the command completes. This has several drawbacks:

- **Redundant network traffic**: Running the same analysis twice re-downloads everything
- **Slow iteration**: Users experimenting with different flags must wait for clone each time
- **CI/CD inefficiency**: Pipelines analyzing the same repo across jobs waste bandwidth

## Decision

### Cache Location

Use the XDG Base Directory specification:
```
$XDG_CACHE_HOME/ferriscan/   # defaults to ~/.cache/ferriscan/
└── repos/
    └── github.com/
        └── user/
            └── repo/
                ├── HEAD/           # default branch clone
                └── v1.0.0/         # specific ref clone
```

The directory structure mirrors the URL for human readability and easy manual cleanup.

### Cache Key

The cache key is derived from:
1. **Host**: `github.com`, `gitlab.com`, etc.
2. **Path**: `user/repo` (normalized, `.git` suffix stripped)
3. **Ref**: The git ref or `HEAD` for default branch

Example mappings:
```
https://github.com/rust-lang/rust       -> github.com/rust-lang/rust/HEAD/
https://github.com/rust-lang/rust@1.75  -> github.com/rust-lang/rust/1.75/
git@github.com:user/repo@main           -> github.com/user/repo/main/
```

### Cache Behavior

1. **Cache hit**: If the directory exists, use it directly (no git operations)
2. **Cache miss**: Clone to the cache location
3. **`--refresh` flag**: Delete cached copy and re-clone
4. **`--no-cache` flag**: Use temp directory (current behavior)

### CLI Changes

```bash
# Uses cache (default)
ferriscan analyze https://github.com/user/repo

# Force fresh clone to cache
ferriscan analyze https://github.com/user/repo --refresh

# Skip cache entirely, use temp dir
ferriscan analyze https://github.com/user/repo --no-cache
```

### Implementation

```rust
pub struct RemoteRepo {
    pub url: String,
    pub git_ref: GitRef,
}

impl RemoteRepo {
    /// Returns the cache directory path for this repo.
    fn cache_path(&self) -> PathBuf {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from(".cache"))
            .join("ferriscan/repos");

        // Parse URL to get host/path
        // e.g., "github.com/user/repo/main"
        cache_dir.join(self.cache_key())
    }

    /// Clone or return cached path.
    pub fn clone_repo(&self, opts: CacheOptions) -> Result<ResolvedRepo, RepoSourceError> {
        if opts.no_cache {
            return self.clone_to_temp();
        }

        let cache_path = self.cache_path();

        if opts.refresh && cache_path.exists() {
            fs::remove_dir_all(&cache_path)?;
        }

        if cache_path.exists() {
            return Ok(ResolvedRepo::from_cached(cache_path));
        }

        self.clone_to_path(&cache_path)?;
        Ok(ResolvedRepo::from_cached(cache_path))
    }
}

pub struct CacheOptions {
    pub refresh: bool,
    pub no_cache: bool,
}
```

### ResolvedRepo Changes

`ResolvedRepo` must distinguish between:
- **Cached**: Path persists, no cleanup on drop
- **Temporary**: `TempDir` cleaned up on drop

```rust
pub struct ResolvedRepo {
    path: PathBuf,
    _temp_dir: Option<TempDir>,  // None for cached repos
}
```

This is already the current structure, so cached repos simply have `_temp_dir: None`.

### Cache Invalidation

The cache is **not automatically invalidated**. Users must use `--refresh` to update.

Rationale:
- Simple implementation
- Predictable behavior
- Users analyzing specific refs (tags, commits) expect immutability
- Users analyzing branches can use `--refresh` when needed

Future consideration: Add `ferriscan cache clear` subcommand.

## Consequences

### Positive
- Dramatically faster repeated analysis of same repo
- Reduced network usage
- Better CI/CD efficiency (cache can be persisted between runs)
- Human-readable cache structure

### Negative
- Disk space usage (mitigated: users can clear cache manually)
- Stale cache for branch refs (mitigated: `--refresh` flag)
- Adds complexity to `RemoteRepo`

### Neutral
- Requires `dirs` crate for XDG compliance
- Cache is per-user, not shared system-wide
