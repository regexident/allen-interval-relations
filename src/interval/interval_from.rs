use core::ops::RangeFrom;

use crate::{
    Contains, Equals, Finishes, Interval, IntervalFull, IntervalTo, Meets, NonEmpty, Overlaps,
    Precedes, Starts,
};

/// An interval only bounded inclusively below (start..).
///
/// The `IntervalFrom { start }` contains all values with `x >= start`.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct IntervalFrom<T> {
    /// The lower bound of the interval (inclusive).
    pub start: T,
}

impl<T> From<RangeFrom<T>> for IntervalFrom<T> {
    fn from(value: RangeFrom<T>) -> Self {
        let RangeFrom { start } = value;
        Self { start }
    }
}

impl<T> From<IntervalFrom<T>> for RangeFrom<T> {
    fn from(value: IntervalFrom<T>) -> Self {
        let IntervalFrom { start } = value;
        Self { start }
    }
}

// IntervalFrom<T> vs. IntervalFull

impl<T> Precedes<NonEmpty<IntervalFull>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Meets<NonEmpty<IntervalFull>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Overlaps<NonEmpty<IntervalFull>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Starts<NonEmpty<IntervalFull>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Contains<NonEmpty<IntervalFull>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Finishes<NonEmpty<IntervalFull>> for NonEmpty<IntervalFrom<T>> {
    #[inline]
    fn finishes(&self, _other: &NonEmpty<IntervalFull>) -> bool {
        true
    }
}

impl<T> Equals<NonEmpty<IntervalFull>> for NonEmpty<IntervalFrom<T>> {}

// IntervalFrom<T> vs. IntervalTo

impl<T> Precedes<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Meets<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Overlaps<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Starts<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Contains<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Finishes<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Equals<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalFrom<T>> {}

// IntervalFrom<T> vs. IntervalFrom

impl<T> Precedes<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Meets<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Overlaps<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Starts<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Contains<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Finishes<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalFrom<T>>
where
    T: PartialOrd,
{
    #[inline]
    fn finishes(&self, other: &NonEmpty<IntervalFrom<T>>) -> bool {
        self.0.start > other.0.start
    }
}

impl<T> Equals<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalFrom<T>>
where
    T: PartialOrd,
{
    #[inline]
    fn equals(&self, other: &NonEmpty<IntervalFrom<T>>) -> bool {
        self.0.start == other.0.start
    }
}

// IntervalFrom<T> vs. Interval

impl<T> Precedes<NonEmpty<Interval<T>>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Meets<NonEmpty<Interval<T>>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Overlaps<NonEmpty<Interval<T>>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Starts<NonEmpty<Interval<T>>> for NonEmpty<IntervalFrom<T>> {}

impl<T> Contains<NonEmpty<Interval<T>>> for NonEmpty<IntervalFrom<T>>
where
    T: PartialOrd,
{
    #[inline]
    fn contains(&self, other: &NonEmpty<Interval<T>>) -> bool {
        self.0.start < other.0.start
    }
}

impl<T> Finishes<NonEmpty<Interval<T>>> for NonEmpty<IntervalFrom<T>>
where
    T: PartialOrd,
{
    #[inline]
    fn finishes(&self, other: &NonEmpty<Interval<T>>) -> bool {
        self.0.start > other.0.start
    }
}

impl<T> Equals<NonEmpty<Interval<T>>> for NonEmpty<IntervalFrom<T>>
where
    T: PartialOrd,
{
    #[inline]
    fn equals(&self, other: &NonEmpty<Interval<T>>) -> bool {
        self.0.start == other.0.start
    }
}
