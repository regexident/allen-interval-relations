/// Create a value from a pair of ranges.
pub trait FromRanges<S, T>: Sized {
    /// Creates a value from a pair of ranges.
    fn from_ranges(s: S, t: T) -> Option<Self>;
}
