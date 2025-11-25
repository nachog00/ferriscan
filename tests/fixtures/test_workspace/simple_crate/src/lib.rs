/// A simple function with no branching.
/// Cyclomatic complexity = 1 (single path)
/// Cognitive complexity = 0 (no nesting/branching)
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Another simple function.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// A function that calls others but has no branching itself.
pub fn calculate(x: i32, y: i32) -> i32 {
    let sum = add(x, y);
    let product = multiply(x, y);
    sum + product
}
