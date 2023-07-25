use std::hash::Hash;

/// An endpoint of a range of time.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Bound<T> {
    /// A finite endpoint.
    ///
    /// Indicates that there is a bound in this direction.
    Bounded(T),
    /// An infinite endpoint.
    ///
    /// Indicates that there is no bound in this direction.
    Unbounded,
}
