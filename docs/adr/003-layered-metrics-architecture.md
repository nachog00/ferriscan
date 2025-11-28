# ADR 003: Layered Metrics Architecture

## Status
Proposed

## Context

The current implementation extracts file-level aggregate metrics directly from `rust-code-analysis` (rac), discarding the per-function granularity that rac provides.

### Problems

1. **Lost granularity**: rac computes metrics per-function, but we only capture file-level aggregates
2. **Tight coupling**: Domain types are shaped by rac's API rather than our needs
3. **Limited extensibility**: Adding function-level features (e.g., a `lint` command to flag functions exceeding complexity thresholds) requires reaching back into rac rather than querying existing domain data

### How rac works

rac returns a `FuncSpace` tree where:
- Root node represents the file
- Children represent nested scopes (impl blocks, functions, closures)
- Each node has its own metrics (cyclomatic, cognitive, MI, etc.)
- Metrics are computed bottom-up and merged into parents

We currently only use the root's merged metrics. The per-function data is discarded.

## Decision

Introduce a three-layer architecture:

```
┌─────────────────────────────────────────┐
│              CLI Layer                  │
│  Arg parsing, output formatting         │
│  Commands operate on domain types only  │
└─────────────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────┐
│            Domain Layer                 │
│  FunctionMetrics, FileMetrics, etc.     │
│  Aggregation, filtering, thresholds     │
│  No knowledge of rac                    │
└─────────────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────┐
│          Extraction Layer               │
│  Parses source code → domain types      │
│  rac is an implementation detail        │
│  Swappable behind a trait               │
└─────────────────────────────────────────┘
```

### Layer Responsibilities

**CLI Layer** (`cli/`)
- Parse arguments, format output (table/JSON)
- Orchestrate calls to domain layer
- No direct interaction with extraction

**Domain Layer** (`domain/`)
- Defines canonical metric types: `FunctionMetrics` → `FileMetrics` → `CrateMetrics` → `WorkspaceMetrics`
- `FunctionMetrics` is the fundamental unit; higher levels aggregate from it
- Contains threshold definitions and violation checking
- Pure logic, no I/O or parsing

**Extraction Layer** (`extraction/`)
- Single responsibility: source code → `Vec<FunctionMetrics>`
- Defines a `MetricsExtractor` trait
- `RcaExtractor` implements it by walking rac's `FuncSpace` tree
- Could be swapped for a different parser without touching other layers

### Data Flow

```
Source files
    ↓
Extraction: recursively walk FuncSpace, emit FunctionMetrics
    ↓
Domain: aggregate functions → files → crates → workspace
    ↓
CLI: query domain types for commands
    - analyze: show aggregates
    - lint: filter functions by thresholds
    - compare: diff two workspace snapshots
```

### Key Design Points

1. **Function-first model**: All metrics originate at function level. File/crate/workspace metrics are derived via aggregation, not extracted separately.

2. **Extraction as trait**: The domain layer depends on an abstraction, not rac directly. This enables testing with mock data and future parser changes.

3. **Aggregation in domain**: We compute sums/averages ourselves rather than relying on rac's pre-merged values. This gives us control and consistency.

## Consequences

### Positive
- Function-level queries become trivial (lint, hotspots, etc.)
- Domain logic testable without parsing real code
- Parser is swappable without touching domain/CLI
- Clear dependency direction: CLI → Domain → Extraction

### Negative
- Storing all functions in memory vs streaming (likely negligible for typical codebases)
- Migration effort to restructure existing code

### Neutral
- More explicit aggregation code, but it's simple and gives us control
