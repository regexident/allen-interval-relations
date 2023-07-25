use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::{Bound, Discrete, Discreteness, EndBound, NonDiscrete, StartBound};

/// An abstraction over Rust's ranges.
pub trait RangeBounds<T, D>
where
    T: Sized,
    D: Discreteness,
{
    /// Start index bound.
    ///
    /// Returns the start value as a [`Bound<T>`].
    fn start_bound(&self) -> StartBound<T, D>;

    /// End index bound.
    ///
    /// Returns the end value as a [`Bound<T>`].
    fn end_bound(&self) -> EndBound<T, D>;
}

impl<T> RangeBounds<T, Discrete> for RangeFrom<T>
where
    T: Clone,
{
    #[inline]
    fn start_bound(&self) -> StartBound<T, Discrete> {
        Bound::Bounded(self.start.clone()).into()
    }

    #[inline]
    fn end_bound(&self) -> EndBound<T, Discrete> {
        Bound::Unbounded.into()
    }
}

impl<T> RangeBounds<T, NonDiscrete> for RangeFrom<T>
where
    T: Clone,
{
    #[inline]
    fn start_bound(&self) -> StartBound<T, NonDiscrete> {
        Bound::Bounded(self.start.clone()).into()
    }

    #[inline]
    fn end_bound(&self) -> EndBound<T, NonDiscrete> {
        Bound::Unbounded.into()
    }
}

impl<T> RangeBounds<T, Discrete> for RangeFull {
    #[inline]
    fn start_bound(&self) -> StartBound<T, Discrete> {
        Bound::Unbounded.into()
    }

    #[inline]
    fn end_bound(&self) -> EndBound<T, Discrete> {
        Bound::Unbounded.into()
    }
}

impl<T> RangeBounds<T, NonDiscrete> for RangeFull {
    #[inline]
    fn start_bound(&self) -> StartBound<T, NonDiscrete> {
        Bound::Unbounded.into()
    }

    #[inline]
    fn end_bound(&self) -> EndBound<T, NonDiscrete> {
        Bound::Unbounded.into()
    }
}

impl<T> RangeBounds<T, Discrete> for Range<T>
where
    T: Clone,
{
    #[inline]
    fn start_bound(&self) -> StartBound<T, Discrete> {
        Bound::Bounded(self.start.clone()).into()
    }

    #[inline]
    fn end_bound(&self) -> EndBound<T, Discrete> {
        Bound::Bounded(self.end.clone()).into()
    }
}

impl<T> RangeBounds<T, NonDiscrete> for RangeInclusive<T>
where
    T: Clone,
{
    #[inline]
    fn start_bound(&self) -> StartBound<T, NonDiscrete> {
        Bound::Bounded(self.start().clone()).into()
    }

    #[inline]
    fn end_bound(&self) -> EndBound<T, NonDiscrete> {
        Bound::Bounded(self.end().clone()).into()
    }
}

impl<T> RangeBounds<T, Discrete> for RangeTo<T>
where
    T: Clone,
{
    #[inline]
    fn start_bound(&self) -> StartBound<T, Discrete> {
        Bound::Unbounded.into()
    }

    #[inline]
    fn end_bound(&self) -> EndBound<T, Discrete> {
        Bound::Bounded(self.end.clone()).into()
    }
}

// `RangeTo` has a discrete end index, hence no impl for `NonDiscrete`.

impl<T> RangeBounds<T, NonDiscrete> for RangeToInclusive<T>
where
    T: Clone,
{
    #[inline]
    fn start_bound(&self) -> StartBound<T, NonDiscrete> {
        Bound::Unbounded.into()
    }

    #[inline]
    fn end_bound(&self) -> EndBound<T, NonDiscrete> {
        Bound::Bounded(self.end.clone()).into()
    }
}

// `RangeToInclusive` has a non-discrete end index, hence no impl for `Discrete`
