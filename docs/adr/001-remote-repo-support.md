# ADR 001: Remote Repository Support

## Status
Accepted

## Context
Users want to analyze remote git repositories without manually cloning them first. This is especially useful for:
- Quick analysis of open source projects
- Comparing different versions/branches of the same repo
- CI/CD pipelines analyzing external dependencies

## Decision

### CLI API Design

All commands use positional arguments for sources and optional paths:

```bash
ferriscan analyze <source> [path]
ferriscan list <source> [path]
ferriscan compare <source1> <source2> [path1] [path2]
```

Where:
- `<source>` is either a local path or a git URL
- `[path]` is an optional relative path within the resolved source (defaults to `.`)
- For `compare`, `path2` defaults to `path1` if not provided

### Source Format

Local paths:
```
.
./src
/absolute/path
../relative/path
```

Remote URLs with optional `@ref` suffix:
```
https://github.com/user/repo
https://github.com/user/repo@main
https://github.com/user/repo@v1.0.0
git@github.com:user/repo
git@github.com:user/repo@feature-branch
```

### Examples

```bash
# Analyze local repo
ferriscan analyze .
ferriscan analyze /path/to/repo src/core

# Analyze remote repo
ferriscan analyze https://github.com/rust-lang/rust
ferriscan analyze https://github.com/rust-lang/rust@1.75.0 compiler/rustc

# Compare versions
ferriscan compare https://github.com/user/repo@v1.0 https://github.com/user/repo@v2.0

# Compare with subpaths
ferriscan compare repo@v1 repo@v2 src/lib
ferriscan compare repo1 repo2 src lib  # different paths
```

### Architecture

```
CLI Layer (cli/args.rs):
  RepoSource
    - Parses input (URL detection, @ref extraction)
    - resolve() -> clones to temp dir if remote
    - Returns local PathBuf

  Final path = resolved_source.join(subdir_path)

Library Layer:
  - Unchanged, receives &Path
  - No knowledge of git or remote repos
```

### Implementation Details

1. **URL Detection**: Check for `https://`, `http://`, `git@`, `ssh://`, `git://` prefixes

2. **Git Clone**: Shell out to `git clone --depth=1` (requires git installed)
   - Use `--branch <ref>` when `@ref` is specified
   - Clone to temp directory via `tempfile` crate

3. **Temp Directory Lifecycle**:
   - `RepoSource.resolve()` returns a `ResolvedRepo` struct
   - `ResolvedRepo` holds `TempDir` to ensure cleanup on drop
   - Analysis completes before `ResolvedRepo` is dropped

4. **ResolvedPath Primitive**:
   - Becomes optional/internal since resolution now happens in `RepoSource`
   - Library functions just take `&Path`

## Consequences

### Positive
- Simple, intuitive CLI for remote analysis
- No complex URL parsing (subdir via separate positional arg)
- Common cases are concise (`compare repo@v1 repo@v2`)
- Clean separation: CLI handles git, library stays pure

### Negative
- Requires `git` to be installed for remote repos
- Shallow clone may miss some history (acceptable for static analysis)
- Temp directories use disk space during analysis

### Neutral
- `ResolvedPath` primitive may become redundant
