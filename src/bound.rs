use std::hash::Hash;

/// An endpoint of a range of time.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Bound<T> {
    Bounded(T),
    Unbounded,
}
