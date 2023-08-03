use crate::{IntervalError, NonEmpty};

/// Create a value from a pair of intervals.
pub trait FromIntervals<S, T>: Sized {
    /// Creates a value from a pair of intervals.
    fn from_intervals(s: &NonEmpty<S>, t: &NonEmpty<T>) -> Self;
}

/// Create a value from a pair of intervals.
pub trait TryFromIntervals<S, T>: Sized {
    /// Creates a value from a pair of intervals.
    fn try_from_intervals(s: &NonEmpty<S>, t: &NonEmpty<T>) -> Result<Self, IntervalError>;
}
