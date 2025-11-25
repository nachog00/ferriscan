/// Deeply nested function - high cognitive complexity.
/// Cognitive complexity penalizes nesting depth.
pub fn deeply_nested_logic(a: i32, b: i32, c: i32) -> i32 {
    let mut result = 0;

    if a > 0 {
        // Nesting level 1
        if b > 0 {
            // Nesting level 2
            if c > 0 {
                // Nesting level 3
                result = a + b + c;
            } else {
                result = a + b;
            }
        } else {
            if c > 0 {
                // Nesting level 2
                result = a + c;
            } else {
                result = a;
            }
        }
    } else {
        if b > 0 {
            // Nesting level 1
            if c > 0 {
                // Nesting level 2
                result = b + c;
            } else {
                result = b;
            }
        } else {
            result = c;
        }
    }

    result
}

/// Function with nested loops.
pub fn matrix_sum(matrix: &[&[i32]]) -> i32 {
    let mut sum = 0;
    for row in matrix {
        for &cell in *row {
            if cell > 0 {
                sum += cell;
            }
        }
    }
    sum
}

/// Function with mixed control flow.
pub fn process_data(items: &[i32], threshold: i32) -> Vec<i32> {
    let mut results = Vec::new();

    for &item in items {
        if item > threshold {
            if item % 2 == 0 {
                results.push(item * 2);
            } else {
                results.push(item);
            }
        } else if item > 0 {
            results.push(item);
        }
    }

    results
}
