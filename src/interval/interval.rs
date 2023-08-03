use core::ops::{Range, RangeInclusive};

use crate::{
    Contains, Equals, Finishes, IntervalFrom, IntervalFull, IntervalTo, Meets, NonEmpty, Overlaps,
    Precedes, Starts,
};

/// A (half-open) interval bounded inclusively below
/// and either exclusively or inclusively above.
///
/// - exclusively above (`start..end`) in discrete domains.
/// - inclusively above (`start..=end`) in continuous domains.
///
/// The interval `Interval { start, end }` contains all values with `start <= x < end`,
/// if `T` is a discrete domain, or `start <= x <= end`, if `T` is a continuous domain.
///
/// It is empty if `start >= end`, if `T` is a discrete domain,
/// or `start > end` if `T` is a continuous domain.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Interval<T> {
    /// The lower bound of the interval (inclusive).
    pub start: T,
    /// The upper bound of the interval (exclusive, or inclusive).
    pub end: T,
}

impl<T> From<Range<T>> for Interval<T> {
    fn from(value: Range<T>) -> Self {
        let Range { start, end } = value;
        Self { start, end }
    }
}

impl<T> From<Interval<T>> for Range<T> {
    fn from(value: Interval<T>) -> Self {
        (value.start)..(value.end)
    }
}

impl<T> From<RangeInclusive<T>> for Interval<T>
where
    T: Copy,
{
    fn from(value: RangeInclusive<T>) -> Self {
        let start = *value.start();
        let end = *value.end();
        Self { start, end }
    }
}

impl<T> From<Interval<T>> for RangeInclusive<T>
where
    T: Copy,
{
    fn from(value: Interval<T>) -> Self {
        (value.start)..=(value.end)
    }
}

// Interval<T> vs. IntervalFull

impl<T> Precedes<NonEmpty<IntervalFull>> for NonEmpty<Interval<T>> {}

impl<T> Meets<NonEmpty<IntervalFull>> for NonEmpty<Interval<T>> {}

impl<T> Overlaps<NonEmpty<IntervalFull>> for NonEmpty<Interval<T>> {}

impl<T> Starts<NonEmpty<IntervalFull>> for NonEmpty<Interval<T>> {}

impl<T> Contains<NonEmpty<IntervalFull>> for NonEmpty<Interval<T>> {}

impl<T> Finishes<NonEmpty<IntervalFull>> for NonEmpty<Interval<T>> {}

impl<T> Equals<NonEmpty<IntervalFull>> for NonEmpty<Interval<T>> {}

// Interval<T> vs. IntervalTo

impl<T> Precedes<NonEmpty<IntervalTo<T>>> for NonEmpty<Interval<T>> {}

impl<T> Meets<NonEmpty<IntervalTo<T>>> for NonEmpty<Interval<T>> {}

impl<T> Overlaps<NonEmpty<IntervalTo<T>>> for NonEmpty<Interval<T>> {}

impl<T> Starts<NonEmpty<IntervalTo<T>>> for NonEmpty<Interval<T>>
where
    T: PartialOrd,
{
    #[inline]
    fn starts(&self, other: &NonEmpty<IntervalTo<T>>) -> bool {
        self.0.end < other.0.end
    }
}

impl<T> Contains<NonEmpty<IntervalTo<T>>> for NonEmpty<Interval<T>> {}

impl<T> Finishes<NonEmpty<IntervalTo<T>>> for NonEmpty<Interval<T>>
where
    T: PartialOrd,
{
    #[inline]
    fn finishes(&self, other: &NonEmpty<IntervalTo<T>>) -> bool {
        self.0.end == other.0.end
    }
}

impl<T> Equals<NonEmpty<IntervalTo<T>>> for NonEmpty<Interval<T>> {}

// Interval<T> vs. IntervalFrom

impl<T> Precedes<NonEmpty<IntervalFrom<T>>> for NonEmpty<Interval<T>>
where
    T: PartialOrd,
{
    #[inline]
    fn precedes(&self, other: &NonEmpty<IntervalFrom<T>>) -> bool {
        self.0.end < other.0.start
    }
}

impl<T> Meets<NonEmpty<IntervalFrom<T>>> for NonEmpty<Interval<T>>
where
    T: PartialOrd,
{
    #[inline]
    fn meets(&self, other: &NonEmpty<IntervalFrom<T>>) -> bool {
        self.0.end == other.0.start
    }
}

impl<T> Overlaps<NonEmpty<IntervalFrom<T>>> for NonEmpty<Interval<T>>
where
    T: PartialOrd,
{
    #[inline]
    fn overlaps(&self, other: &NonEmpty<IntervalFrom<T>>) -> bool {
        (self.0.start < other.0.start) && (other.0.start < self.0.end)
    }
}

impl<T> Starts<NonEmpty<IntervalFrom<T>>> for NonEmpty<Interval<T>>
where
    T: PartialOrd,
{
    #[inline]
    fn starts(&self, other: &NonEmpty<IntervalFrom<T>>) -> bool {
        self.0.start == other.0.start
    }
}

impl<T> Contains<NonEmpty<IntervalFrom<T>>> for NonEmpty<Interval<T>> {}

impl<T> Finishes<NonEmpty<IntervalFrom<T>>> for NonEmpty<Interval<T>>
where
    T: PartialOrd,
{
    #[inline]
    fn finishes(&self, other: &NonEmpty<IntervalFrom<T>>) -> bool {
        self.0.start > other.0.start
    }
}

impl<T> Equals<NonEmpty<IntervalFrom<T>>> for NonEmpty<Interval<T>> {}

// Interval<T> vs. Interval

impl<T> Precedes<NonEmpty<Interval<T>>> for NonEmpty<Interval<T>>
where
    T: PartialOrd,
{
    #[inline]
    fn precedes(&self, other: &NonEmpty<Interval<T>>) -> bool {
        self.0.end < other.0.start
    }
}

impl<T> Meets<NonEmpty<Interval<T>>> for NonEmpty<Interval<T>>
where
    T: PartialOrd,
{
    #[inline]
    fn meets(&self, other: &NonEmpty<Interval<T>>) -> bool {
        self.0.end == other.0.start
    }
}

impl<T> Overlaps<NonEmpty<Interval<T>>> for NonEmpty<Interval<T>>
where
    T: PartialOrd,
{
    #[inline]
    fn overlaps(&self, other: &NonEmpty<Interval<T>>) -> bool {
        (self.0.start < other.0.start) && (other.0.start < self.0.end) && (self.0.end < other.0.end)
    }
}

impl<T> Starts<NonEmpty<Interval<T>>> for NonEmpty<Interval<T>>
where
    T: PartialOrd,
{
    #[inline]
    fn starts(&self, other: &NonEmpty<Interval<T>>) -> bool {
        (self.0.start == other.0.start) && (self.0.end < other.0.end)
    }
}

impl<T> Contains<NonEmpty<Interval<T>>> for NonEmpty<Interval<T>>
where
    T: PartialOrd,
{
    #[inline]
    fn contains(&self, other: &NonEmpty<Interval<T>>) -> bool {
        (self.0.start < other.0.start) && (self.0.end > other.0.end)
    }
}

impl<T> Finishes<NonEmpty<Interval<T>>> for NonEmpty<Interval<T>>
where
    T: PartialOrd,
{
    #[inline]
    fn finishes(&self, other: &NonEmpty<Interval<T>>) -> bool {
        (self.0.start > other.0.start) && (self.0.end == other.0.end)
    }
}

impl<T> Equals<NonEmpty<Interval<T>>> for NonEmpty<Interval<T>>
where
    T: PartialOrd,
{
    #[inline]
    fn equals(&self, other: &NonEmpty<Interval<T>>) -> bool {
        (self.0.start == other.0.start) && (self.0.end == other.0.end)
    }
}
