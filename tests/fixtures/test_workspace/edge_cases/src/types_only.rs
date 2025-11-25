/// A struct with no methods - tests files without function metrics.
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// An enum without match arms in functions.
pub enum Color {
    Red,
    Green,
    Blue,
}

/// A type alias.
pub type Coordinate = (f64, f64);

/// A constant.
pub const MAX_VALUE: i32 = 100;
