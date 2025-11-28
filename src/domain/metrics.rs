use serde::{Deserialize, Serialize};

/// The kind of function-like construct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionKind {
    Function,
    Closure,
}

/// Metrics for a single function, method, or closure.
///
/// This is the fundamental unit of measurement. All higher-level metrics
/// (file, crate, workspace) are aggregated from function metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionMetrics {
    /// Function name (if available).
    pub name: Option<String>,

    /// Start line in the file (1-indexed).
    pub start_line: usize,

    /// End line in the file (1-indexed).
    pub end_line: usize,

    /// What kind of function-like construct this is.
    pub kind: FunctionKind,

    // Size metrics
    /// Source lines of code.
    pub sloc: f64,

    /// Physical lines of code.
    pub ploc: f64,

    /// Logical lines of code.
    pub lloc: f64,

    /// Comment lines of code.
    pub cloc: f64,

    /// Blank lines.
    pub blank: f64,

    // Complexity metrics
    /// Cyclomatic complexity.
    pub cyclomatic: f64,

    /// Cognitive complexity.
    pub cognitive: f64,

    // Halstead metrics
    pub halstead_difficulty: f64,
    pub halstead_effort: f64,
    pub halstead_bugs: f64,

    /// Maintainability index (Visual Studio variant).
    pub mi: f64,

    /// Number of arguments.
    pub nargs: usize,

    /// Number of exit points.
    pub nexits: usize,
}

/// Metrics for a file, aggregated from its functions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetrics {
    /// Relative path of the file.
    pub path: String,

    /// All functions in this file.
    pub functions: Vec<FunctionMetrics>,

    // Aggregated size metrics
    pub sloc: f64,
    pub ploc: f64,
    pub lloc: f64,
    pub cloc: f64,
    pub blank: f64,

    // Aggregated complexity
    pub cyclomatic_sum: f64,
    pub cyclomatic_avg: f64,
    pub cognitive_sum: f64,
    pub cognitive_avg: f64,

    // Aggregated Halstead
    pub halstead_difficulty_avg: f64,
    pub halstead_effort_sum: f64,
    pub halstead_bugs_sum: f64,

    /// Average maintainability index.
    pub mi_avg: f64,

    // Counts
    pub nargs_sum: usize,
    pub nargs_avg: f64,
    pub nexits_sum: usize,
}

impl FileMetrics {
    /// Construct file metrics by aggregating from function metrics.
    pub fn from_functions(path: String, functions: Vec<FunctionMetrics>) -> Self {
        let n = functions.len();

        if n == 0 {
            return Self {
                path,
                functions,
                sloc: 0.0,
                ploc: 0.0,
                lloc: 0.0,
                cloc: 0.0,
                blank: 0.0,
                cyclomatic_sum: 0.0,
                cyclomatic_avg: 0.0,
                cognitive_sum: 0.0,
                cognitive_avg: 0.0,
                halstead_difficulty_avg: 0.0,
                halstead_effort_sum: 0.0,
                halstead_bugs_sum: 0.0,
                mi_avg: 0.0,
                nargs_sum: 0,
                nargs_avg: 0.0,
                nexits_sum: 0,
            };
        }

        let n_f64 = n as f64;

        let sloc: f64 = functions.iter().map(|f| f.sloc).sum();
        let ploc: f64 = functions.iter().map(|f| f.ploc).sum();
        let lloc: f64 = functions.iter().map(|f| f.lloc).sum();
        let cloc: f64 = functions.iter().map(|f| f.cloc).sum();
        let blank: f64 = functions.iter().map(|f| f.blank).sum();

        let cyclomatic_sum: f64 = functions.iter().map(|f| f.cyclomatic).sum();
        let cognitive_sum: f64 = functions.iter().map(|f| f.cognitive).sum();
        let halstead_difficulty_avg: f64 =
            functions.iter().map(|f| f.halstead_difficulty).sum::<f64>() / n_f64;
        let halstead_effort_sum: f64 = functions.iter().map(|f| f.halstead_effort).sum();
        let halstead_bugs_sum: f64 = functions.iter().map(|f| f.halstead_bugs).sum();
        let mi_avg: f64 = functions.iter().map(|f| f.mi).sum::<f64>() / n_f64;

        let nargs_sum: usize = functions.iter().map(|f| f.nargs).sum();
        let nexits_sum: usize = functions.iter().map(|f| f.nexits).sum();

        Self {
            path,
            functions,
            sloc,
            ploc,
            lloc,
            cloc,
            blank,
            cyclomatic_sum,
            cyclomatic_avg: cyclomatic_sum / n_f64,
            cognitive_sum,
            cognitive_avg: cognitive_sum / n_f64,
            halstead_difficulty_avg,
            halstead_effort_sum,
            halstead_bugs_sum,
            mi_avg,
            nargs_sum,
            nargs_avg: nargs_sum as f64 / n_f64,
            nexits_sum,
        }
    }

    /// Number of functions in this file.
    pub fn function_count(&self) -> usize {
        self.functions.len()
    }

    /// Number of closures in this file.
    pub fn closure_count(&self) -> usize {
        self.functions
            .iter()
            .filter(|f| f.kind == FunctionKind::Closure)
            .count()
    }
}

/// Metrics for a crate, aggregated from its files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateMetrics {
    /// Crate name.
    pub name: String,

    /// All files in this crate.
    pub files: Vec<FileMetrics>,

    // Aggregated totals
    pub total_sloc: f64,
    pub total_ploc: f64,
    pub total_lloc: f64,
    pub total_cloc: f64,
    pub total_blank: f64,
    pub total_functions: usize,
    pub total_closures: usize,

    // Averages
    pub avg_file_sloc: f64,
    pub avg_functions_per_file: f64,
    pub avg_cyclomatic: f64,
    pub avg_cognitive: f64,
    pub avg_mi: f64,
    pub avg_halstead_difficulty: f64,
}

impl CrateMetrics {
    /// Construct crate metrics by aggregating from file metrics.
    pub fn from_files(name: String, files: Vec<FileMetrics>) -> Self {
        let n = files.len();

        if n == 0 {
            return Self {
                name,
                files,
                total_sloc: 0.0,
                total_ploc: 0.0,
                total_lloc: 0.0,
                total_cloc: 0.0,
                total_blank: 0.0,
                total_functions: 0,
                total_closures: 0,
                avg_file_sloc: 0.0,
                avg_functions_per_file: 0.0,
                avg_cyclomatic: 0.0,
                avg_cognitive: 0.0,
                avg_mi: 0.0,
                avg_halstead_difficulty: 0.0,
            };
        }

        let total_sloc: f64 = files.iter().map(|f| f.sloc).sum();
        let total_ploc: f64 = files.iter().map(|f| f.ploc).sum();
        let total_lloc: f64 = files.iter().map(|f| f.lloc).sum();
        let total_cloc: f64 = files.iter().map(|f| f.cloc).sum();
        let total_blank: f64 = files.iter().map(|f| f.blank).sum();
        let total_functions: usize = files.iter().map(|f| f.function_count()).sum();
        let total_closures: usize = files.iter().map(|f| f.closure_count()).sum();

        let n_f64 = n as f64;

        // Weighted averages by SLOC for complexity metrics
        let avg_cyclomatic = weighted_avg(&files, |f| f.cyclomatic_avg, |f| f.sloc);
        let avg_cognitive = weighted_avg(&files, |f| f.cognitive_avg, |f| f.sloc);
        let avg_mi = weighted_avg(&files, |f| f.mi_avg, |f| f.sloc);
        let avg_halstead_difficulty =
            weighted_avg(&files, |f| f.halstead_difficulty_avg, |f| f.sloc);

        Self {
            name,
            files,
            total_sloc,
            total_ploc,
            total_lloc,
            total_cloc,
            total_blank,
            total_functions,
            total_closures,
            avg_file_sloc: total_sloc / n_f64,
            avg_functions_per_file: total_functions as f64 / n_f64,
            avg_cyclomatic,
            avg_cognitive,
            avg_mi,
            avg_halstead_difficulty,
        }
    }

    /// Number of files in this crate.
    pub fn file_count(&self) -> usize {
        self.files.len()
    }
}

/// Metrics for a workspace, aggregated from its crates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMetrics {
    /// All crates in this workspace.
    pub crates: Vec<CrateMetrics>,

    // Aggregated totals
    pub total_sloc: f64,
    pub total_functions: usize,

    // Averages
    pub avg_cyclomatic: f64,
    pub avg_cognitive: f64,
    pub avg_mi: f64,
}

impl WorkspaceMetrics {
    /// Construct workspace metrics by aggregating from crate metrics.
    pub fn from_crates(crates: Vec<CrateMetrics>) -> Self {
        if crates.is_empty() {
            return Self {
                crates,
                total_sloc: 0.0,
                total_functions: 0,
                avg_cyclomatic: 0.0,
                avg_cognitive: 0.0,
                avg_mi: 0.0,
            };
        }

        let total_sloc: f64 = crates.iter().map(|c| c.total_sloc).sum();
        let total_functions: usize = crates.iter().map(|c| c.total_functions).sum();

        // Weighted averages by SLOC
        let avg_cyclomatic = weighted_avg(&crates, |c| c.avg_cyclomatic, |c| c.total_sloc);
        let avg_cognitive = weighted_avg(&crates, |c| c.avg_cognitive, |c| c.total_sloc);
        let avg_mi = weighted_avg(&crates, |c| c.avg_mi, |c| c.total_sloc);

        Self {
            crates,
            total_sloc,
            total_functions,
            avg_cyclomatic,
            avg_cognitive,
            avg_mi,
        }
    }

    /// Total number of crates.
    pub fn crate_count(&self) -> usize {
        self.crates.len()
    }

    /// Total number of files across all crates.
    pub fn file_count(&self) -> usize {
        self.crates.iter().map(|c| c.file_count()).sum()
    }
}

/// Compute weighted average, skipping NaN values.
fn weighted_avg<T, V, W>(items: &[T], value_fn: V, weight_fn: W) -> f64
where
    V: Fn(&T) -> f64,
    W: Fn(&T) -> f64,
{
    let (sum, weight) = items
        .iter()
        .filter(|item| !value_fn(item).is_nan())
        .fold((0.0, 0.0), |(sum, weight), item| {
            let w = weight_fn(item);
            (sum + value_fn(item) * w, weight + w)
        });

    if weight > 0.0 {
        sum / weight
    } else {
        0.0
    }
}
