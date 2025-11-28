pub mod crate_metrics;
pub mod file_metrics;
pub mod function_metrics;
pub mod workspace_metrics;

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
