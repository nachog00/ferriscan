pub mod nested;

/// Function with multiple branches (higher cyclomatic complexity).
/// Each if/else branch adds to cyclomatic complexity.
pub fn classify_number(n: i32) -> &'static str {
    if n < 0 {
        "negative"
    } else if n == 0 {
        "zero"
    } else if n < 10 {
        "small"
    } else if n < 100 {
        "medium"
    } else {
        "large"
    }
}

/// Function with a match statement (adds to cyclomatic).
pub fn day_type(day: u8) -> &'static str {
    match day {
        1 | 7 => "weekend",
        2..=6 => "weekday",
        _ => "invalid",
    }
}

/// Function with loop and early return.
pub fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

/// Function with multiple conditions combined.
pub fn validate_input(value: i32, min: i32, max: i32) -> Result<i32, &'static str> {
    if value < min {
        Err("too small")
    } else if value > max {
        Err("too large")
    } else if value == 0 {
        Err("cannot be zero")
    } else {
        Ok(value)
    }
}
