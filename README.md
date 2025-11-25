# ferriscan

Code metrics for Rust projects. Analyzes complexity, maintainability, and size across crates and workspaces.

## Install

```bash
cargo install --path .
```

## Usage

```bash
# Analyze current directory
ferriscan analyze

# Analyze a specific path
ferriscan analyze /path/to/project

# Analyze a remote repo
ferriscan analyze https://github.com/user/repo
ferriscan analyze https://github.com/user/repo@v1.0.0

# Compare two versions
ferriscan compare https://github.com/user/repo@v1.0 https://github.com/user/repo@v2.0

# List crates in a workspace
ferriscan list .
```

## Metrics

| Metric | Description |
|--------|-------------|
| SLOC | Source lines of code |
| Cyclomatic | Branch complexity (lower is simpler) |
| Cognitive | Nesting/readability complexity (lower is better) |
| MI | Maintainability index (higher is better) |
| Halstead | Effort, difficulty, estimated bugs |

## Output Formats

```bash
ferriscan analyze --format table      # default
ferriscan analyze --format json
ferriscan analyze --format json-pretty
```

## Warnings

Check files against thresholds:

```bash
ferriscan analyze -w
ferriscan analyze -w --max-sloc 300 --max-cyclomatic 8 --min-mi 30
```

## Verbose Output

Show top files by size, complexity, and worst maintainability:

```bash
ferriscan analyze -v
```

## Library

```rust
use ferriscan::{workspace, analyzer};

let crates = workspace::discover_crates(path)?;
let report = analyzer::analyze_workspace(&crates);
```

## License

MIT
