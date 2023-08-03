use crate::{
    interval::{Interval, IntervalFrom, IntervalFull, IntervalTo},
    NonEmpty,
};

/// An endpoint of an interval of time.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
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

/// The endpoints of an interval of time.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bounds<T> {
    /// Start index bound.
    ///
    /// Returns the start value as a Bound.
    pub start: Bound<T>,

    /// End index bound.
    ///
    /// Returns the end value as a Bound.
    pub end: Bound<T>,
}

/// `IntervalBounds` is implemented by the crate's built-in interval types.
pub trait IntervalBounds<T> {
    /// Start index bound.
    ///
    /// Returns the start value as a [`Bound<T>`].
    fn start_bound(&self) -> Bound<T>;

    /// End index bound.
    ///
    /// Returns the end value as a [`Bound<T>`].
    fn end_bound(&self) -> Bound<T>;

    /// Index bounds.
    ///
    /// Returns the start end end bounds as a [`Bounds<T>`].
    fn bounds(&self) -> Bounds<T> {
        Bounds {
            start: self.start_bound(),
            end: self.end_bound(),
        }
    }
}

impl<I, T> IntervalBounds<T> for NonEmpty<I>
where
    I: IntervalBounds<T>,
{
    fn start_bound(&self) -> Bound<T> {
        self.0.start_bound()
    }

    fn end_bound(&self) -> Bound<T> {
        self.0.end_bound()
    }
}

impl<T> IntervalBounds<T> for Interval<T>
where
    T: Copy,
{
    fn start_bound(&self) -> Bound<T> {
        Bound::Bounded(self.start)
    }

    fn end_bound(&self) -> Bound<T> {
        Bound::Bounded(self.end)
    }
}

impl<T> IntervalBounds<T> for IntervalFrom<T>
where
    T: Copy,
{
    fn start_bound(&self) -> Bound<T> {
        Bound::Bounded(self.start)
    }

    fn end_bound(&self) -> Bound<T> {
        Bound::Unbounded
    }
}

impl<T> IntervalBounds<T> for IntervalTo<T>
where
    T: Copy,
{
    fn start_bound(&self) -> Bound<T> {
        Bound::Unbounded
    }

    fn end_bound(&self) -> Bound<T> {
        Bound::Bounded(self.end)
    }
}

impl<T> IntervalBounds<T> for IntervalFull {
    fn start_bound(&self) -> Bound<T> {
        Bound::Unbounded
    }

    fn end_bound(&self) -> Bound<T> {
        Bound::Unbounded
    }
}
