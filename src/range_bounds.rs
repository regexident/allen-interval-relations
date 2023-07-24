use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::{Bound, Discrete, Discreteness, EndBound, NonDiscrete, StartBound};

pub trait RangeBounds<T, D>
where
    T: Sized,
    D: Discreteness,
{
    fn start_bound(&self) -> StartBound<T, D>;
    fn end_bound(&self) -> EndBound<T, D>;
}

impl<T> RangeBounds<T, Discrete> for RangeFrom<T>
where
    T: Clone,
{
    fn start_bound(&self) -> StartBound<T, Discrete> {
        Bound::Bounded(self.start.clone()).into()
    }

    fn end_bound(&self) -> EndBound<T, Discrete> {
        Bound::Unbounded.into()
    }
}

impl<T> RangeBounds<T, NonDiscrete> for RangeFrom<T>
where
    T: Clone,
{
    fn start_bound(&self) -> StartBound<T, NonDiscrete> {
        Bound::Bounded(self.start.clone()).into()
    }

    fn end_bound(&self) -> EndBound<T, NonDiscrete> {
        Bound::Unbounded.into()
    }
}

impl<T> RangeBounds<T, Discrete> for RangeFull {
    fn start_bound(&self) -> StartBound<T, Discrete> {
        Bound::Unbounded.into()
    }

    fn end_bound(&self) -> EndBound<T, Discrete> {
        Bound::Unbounded.into()
    }
}

impl<T> RangeBounds<T, NonDiscrete> for RangeFull {
    fn start_bound(&self) -> StartBound<T, NonDiscrete> {
        Bound::Unbounded.into()
    }

    fn end_bound(&self) -> EndBound<T, NonDiscrete> {
        Bound::Unbounded.into()
    }
}

impl<T> RangeBounds<T, Discrete> for Range<T>
where
    T: Clone,
{
    fn start_bound(&self) -> StartBound<T, Discrete> {
        Bound::Bounded(self.start.clone()).into()
    }

    fn end_bound(&self) -> EndBound<T, Discrete> {
        Bound::Bounded(self.end.clone()).into()
    }
}

impl<T> RangeBounds<T, NonDiscrete> for RangeInclusive<T>
where
    T: Clone,
{
    fn start_bound(&self) -> StartBound<T, NonDiscrete> {
        Bound::Bounded(self.start().clone()).into()
    }

    fn end_bound(&self) -> EndBound<T, NonDiscrete> {
        Bound::Bounded(self.end().clone()).into()
    }
}

impl<T> RangeBounds<T, Discrete> for RangeTo<T>
where
    T: Clone,
{
    fn start_bound(&self) -> StartBound<T, Discrete> {
        Bound::Unbounded.into()
    }

    fn end_bound(&self) -> EndBound<T, Discrete> {
        Bound::Bounded(self.end.clone()).into()
    }
}

// `RangeTo` has a discrete end index, hence no impl for `NonDiscrete`.

impl<T> RangeBounds<T, NonDiscrete> for RangeToInclusive<T>
where
    T: Clone,
{
    fn start_bound(&self) -> StartBound<T, NonDiscrete> {
        Bound::Unbounded.into()
    }

    fn end_bound(&self) -> EndBound<T, NonDiscrete> {
        Bound::Bounded(self.end.clone()).into()
    }
}

// `RangeToInclusive` has a non-discrete end index, hence no impl for `Discrete`
