use std::path::PathBuf;

use ferriscan::domain::analysis::{analyze_crate, analyze_workspace};
use ferriscan::extraction::rca::RcaExtractor;
use ferriscan::workspace::{discover_crates, CrateInfo};

fn fixtures_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/test_workspace")
}

fn get_crate_info(name: &str) -> CrateInfo {
    CrateInfo {
        name: name.to_string(),
        path: fixtures_path().join(name),
    }
}

#[test]
fn test_workspace_discovery() {
    let crates = discover_crates(&fixtures_path()).unwrap();

    assert_eq!(crates.len(), 3, "should discover 3 crates");

    let names: Vec<_> = crates.iter().map(|c| c.name.as_str()).collect();
    assert!(names.contains(&"simple_crate"));
    assert!(names.contains(&"complex_crate"));
    assert!(names.contains(&"edge_cases"));
}

#[test]
fn test_simple_crate_metrics() {
    let extractor = RcaExtractor;
    let crate_info = get_crate_info("simple_crate");
    let report = analyze_crate(&extractor, &crate_info);

    assert_eq!(report.name, "simple_crate");
    assert_eq!(report.file_count(), 1);
    // SLOC now counts only function bodies, not file-level code (imports, etc.)
    assert_eq!(report.total_sloc, 11.0);
    assert_eq!(report.total_functions, 3);

    // Simple functions have cyclomatic complexity of 1
    assert_eq!(report.avg_cyclomatic, 1.0);
    // No branching = 0 cognitive complexity
    assert_eq!(report.avg_cognitive, 0.0);

    // Check file-level metrics
    assert_eq!(report.files.len(), 1);
    let file = &report.files[0];
    assert!(file.path.ends_with("lib.rs"));
    assert_eq!(file.cyclomatic_sum, 3.0); // 3 functions with complexity 1 each
    assert_eq!(file.cognitive_sum, 0.0);
}

#[test]
fn test_complex_crate_metrics() {
    let extractor = RcaExtractor;
    let crate_info = get_crate_info("complex_crate");
    let report = analyze_crate(&extractor, &crate_info);

    assert_eq!(report.name, "complex_crate");
    assert_eq!(report.file_count(), 2);
    // SLOC now counts only function bodies
    assert_eq!(report.total_sloc, 104.0);

    // Complex crate should have higher complexity than simple
    assert!(
        report.avg_cyclomatic > 1.0,
        "complex crate should have cyclomatic > 1, got {}",
        report.avg_cyclomatic
    );
    assert!(
        report.avg_cognitive > 0.0,
        "complex crate should have cognitive > 0, got {}",
        report.avg_cognitive
    );

    // nested.rs should have higher cognitive complexity due to deep nesting
    let nested_file = report.files.iter().find(|f| f.path.contains("nested")).unwrap();
    assert!(
        nested_file.cognitive_avg > 5.0,
        "nested.rs should have high cognitive complexity, got {}",
        nested_file.cognitive_avg
    );
}

#[test]
fn test_edge_cases_empty_file() {
    let extractor = RcaExtractor;
    let crate_info = get_crate_info("edge_cases");
    let report = analyze_crate(&extractor, &crate_info);

    assert_eq!(report.name, "edge_cases");
    assert_eq!(report.file_count(), 3);

    // empty.rs has no functions, so it will have 0 for everything
    let empty_file = report.files.iter().find(|f| f.path.contains("empty")).unwrap();
    assert_eq!(empty_file.function_count(), 0);
    assert_eq!(empty_file.cyclomatic_sum, 0.0);

    // Crate averages should still be valid numbers
    assert!(
        !report.avg_cyclomatic.is_nan(),
        "crate avg_cyclomatic should not be NaN"
    );
    assert!(
        !report.avg_cognitive.is_nan(),
        "crate avg_cognitive should not be NaN"
    );
    assert!(
        !report.avg_mi.is_nan(),
        "crate avg_mi should not be NaN"
    );
}

#[test]
fn test_types_only_file() {
    let extractor = RcaExtractor;
    let crate_info = get_crate_info("edge_cases");
    let report = analyze_crate(&extractor, &crate_info);

    // types_only.rs has structs/enums but no functions
    let types_file = report.files.iter().find(|f| f.path.contains("types_only")).unwrap();
    assert_eq!(types_file.function_count(), 0);
}

#[test]
fn test_full_workspace_analysis() {
    let extractor = RcaExtractor;
    let crates = discover_crates(&fixtures_path()).unwrap();
    let report = analyze_workspace(&extractor, &crates);

    assert_eq!(report.crate_count(), 3);
    assert_eq!(report.file_count(), 6);
    // SLOC now counts only function bodies
    assert_eq!(report.total_sloc, 118.0);

    // Workspace averages should be weighted by SLOC
    // complex_crate has most SLOC (104) so it dominates the average
    assert!(
        report.avg_cyclomatic > 2.0,
        "workspace avg_cyclomatic should be > 2 due to complex_crate weight"
    );

    // All averages should be valid (no NaN propagation)
    assert!(!report.avg_cyclomatic.is_nan());
    assert!(!report.avg_cognitive.is_nan());
    assert!(!report.avg_mi.is_nan());
}

#[test]
fn test_maintainability_index_ranges() {
    let extractor = RcaExtractor;
    let crates = discover_crates(&fixtures_path()).unwrap();
    let report = analyze_workspace(&extractor, &crates);

    for crate_report in &report.crates {
        // MI should be in valid range (typically 0-100, but can exceed)
        if !crate_report.avg_mi.is_nan() && crate_report.total_sloc > 0.0 {
            assert!(
                crate_report.avg_mi >= 0.0,
                "MI should be >= 0, got {} for {}",
                crate_report.avg_mi,
                crate_report.name
            );
        }
    }

    // simple_crate should have better (higher) MI than complex_crate
    let simple = report.crates.iter().find(|c| c.name == "simple_crate").unwrap();
    let complex = report.crates.iter().find(|c| c.name == "complex_crate").unwrap();

    assert!(
        simple.avg_mi > complex.avg_mi,
        "simple_crate MI ({}) should be > complex_crate MI ({})",
        simple.avg_mi,
        complex.avg_mi
    );
}

#[test]
fn test_function_level_access() {
    let extractor = RcaExtractor;
    let crate_info = get_crate_info("simple_crate");
    let report = analyze_crate(&extractor, &crate_info);

    // We now have access to individual functions
    let file = &report.files[0];
    assert!(file.functions.len() > 0, "should have functions");

    for func in &file.functions {
        // Each function should have a name and valid metrics
        assert!(func.name.is_some(), "function should have a name");
        assert!(func.cyclomatic >= 1.0, "cyclomatic should be >= 1");
        assert!(func.start_line > 0, "start_line should be > 0");
        assert!(func.end_line >= func.start_line, "end_line should be >= start_line");
    }
}
