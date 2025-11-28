# ADR 004: Strongly-Typed Domain Metrics

## Status
Proposed

## Context

Metrics from `rust-code-analysis` arrive as `f64` values. The current codebase propagates these raw floats through domain types, leading to:

1. **No semantic distinction**: `sloc: f64` and `cyclomatic: f64` are interchangeable at the type level, but adding them is nonsensical
2. **Scattered validation**: Ad-hoc checks for non-negative values, NaN handling, and bounds appear throughout the code
3. **Unclear relationships**: When `FileMetrics.cyclomatic_avg` and `CrateMetrics.avg_cyclomatic` are both `f64`, it's not obvious they represent the same concept at different aggregation levels
4. **Lossy conversions**: Some metrics are naturally integers (line counts) but stored as floats

We want **Parse, Don't Validate**: by the time data enters the domain layer, it should already be in types that make invalid states unrepresentable.

## Decision

Introduce newtype wrappers for each metric category. Validation happens once at the extraction boundary; domain logic operates on types with built-in guarantees.

### Metric Categories

**Counts** — Non-negative integers (line counts, function counts, arguments):
- `Sloc`, `Ploc`, `Lloc`, `Cloc`, `BlankLines`
- `FunctionCount`, `ClosureCount`
- `ArgCount`, `ExitCount`
- `LineNumber`

These wrap `u32` or `u64`. Construction validates non-negativity (relevant when parsing from rac's f64).

**Complexity Scores** — Non-negative measures:
- `CyclomaticComplexity`
- `CognitiveComplexity`

These wrap a non-negative numeric type. Cyclomatic is always ≥1 for any function; cognitive ≥0.

**Indices** — Bounded or semi-bounded ranges:
- `MaintainabilityIndex` — Typically 0–100, though can go negative for very poor code
- `HalsteadDifficulty`, `HalsteadEffort`, `HalsteadBugs` — Non-negative floats

**Averages** — Derived values, semantically linked to their base metric:
- `Average<T>` generic wrapper, or specific types like `AvgCyclomatic`
- Makes it clear that a value is a derived average, not a raw measurement

### Guarantees by Construction

Each type enforces its invariants at creation:

```
Extraction layer (rac boundary):
  f64 from rac → TryInto<Sloc> → Result<Sloc, SlocError>

Domain layer:
  Sloc is guaranteed valid, no further checks needed
```

### Self-Describing Errors

Each newtype defines its own error types. Two categories:

**Validation errors** — Occur at construction, when raw data violates invariants:

```
SlocValidationError::Negative(-5.0)
SlocValidationError::NaN
CyclomaticValidationError::BelowMinimum(0.5)  // must be ≥1
```

These bubble up through the extraction layer. A file-level extraction error can wrap function-level metric errors, preserving context:

```
ExtractionError::InvalidMetric {
    file: "src/foo.rs",
    function: "parse_input",
    line: 42,
    source: CyclomaticValidationError::NaN,
}
```

**Method errors** — Occur when operations on valid values fail:

```
// Division that would produce invalid result
SlocOpError::DivisionByZero

// Subtraction that would underflow an unsigned count
SlocOpError::WouldUnderflow { lhs: Sloc(5), rhs: Sloc(10) }

// Weighted average with no weights
AvgError::EmptyInput
```

Method errors can be consumed by domain logic or other newtypes that compose operations. They describe failures in terms of the domain, not raw numeric issues.

**Marker trait for consistency** — A trait enforces that each newtype declares its error types:

```
trait DomainPrimitive: Sized {
    type ValidationError: Error;
    type OpError: Error;
}
```

This applies beyond metrics — identifiers, paths, locations, and other validated domain values follow the same pattern. The trait guarantees every domain primitive has a clear mapping to its error types, enabling generic error handling and making the pattern discoverable. Composite operations can constrain on `T: DomainPrimitive` and work uniformly with any primitive's errors.

The domain layer handles method errors as needed, but never validation errors — by the time data arrives, it's valid by construction.

Benefits:
- **No NaN in domain**: Extraction layer rejects or handles NaN before constructing domain types
- **No negative counts**: `Sloc` cannot represent -5
- **Type-safe aggregation**: Summing `Sloc` values yields `Sloc`, not raw `f64`
- **Clear semantics**: Function signature `fn average(values: &[CyclomaticComplexity]) -> AvgCyclomatic` is self-documenting

### Cross-Layer Consistency

The same newtype appears at every aggregation level:

```
FunctionMetrics {
    cyclomatic: CyclomaticComplexity,
    ...
}

FileMetrics {
    cyclomatic_sum: CyclomaticComplexity,  // same type, sum of functions
    cyclomatic_avg: AvgCyclomatic,
    ...
}

CrateMetrics {
    cyclomatic_avg: AvgCyclomatic,  // clearly the same concept
    ...
}
```

This makes relationships explicit: if two fields share a type, they're meaningfully comparable.

### Threshold Comparisons

Thresholds use the same types:

```
Thresholds {
    max_cyclomatic: CyclomaticComplexity,
    max_cognitive: CognitiveComplexity,
    min_mi: MaintainabilityIndex,
}
```

Comparisons become type-safe: you can't accidentally compare cyclomatic against a cognitive threshold.

## Consequences

### Positive
- Invalid states unrepresentable in domain layer
- No scattered NaN/bounds checks — handled once at extraction
- Self-documenting types reveal intent and relationships
- Compile-time prevention of nonsensical operations (adding SLOC to complexity)
- Aggregation logic is obviously correct when types align

### Negative
- More types to define and maintain
- Conversion boilerplate at extraction boundary
- Some ergonomic friction (explicit conversions for display/serialization)

### Neutral
- Existing `FileCount` pattern extends naturally to other metrics
- `derive_more` crate reduces newtype boilerplate
