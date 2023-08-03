use core::ops::{RangeTo, RangeToInclusive};

use crate::{
    Contains, Equals, Finishes, Interval, IntervalFrom, IntervalFull, Meets, NonEmpty, Overlaps,
    Precedes, Starts,
};

/// An interval only bounded exclusively, or inclusively above (`..end`, or `..=end`).
///
/// The RangeTo ..end contains all values with `x < end`, if `T` is a discrete domain,
/// or `x <= end`, if `T` is a continuous domain.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct IntervalTo<T> {
    /// The upper bound of the interval (exclusive, or inclusive).
    pub end: T,
}

impl<T> From<RangeTo<T>> for IntervalTo<T> {
    fn from(value: RangeTo<T>) -> Self {
        Self { end: value.end }
    }
}

impl<T> From<IntervalTo<T>> for RangeTo<T> {
    fn from(value: IntervalTo<T>) -> Self {
        ..(value.end)
    }
}

impl<T> From<RangeToInclusive<T>> for IntervalTo<T> {
    fn from(value: RangeToInclusive<T>) -> Self {
        let end = value.end;
        Self { end }
    }
}

impl<T> From<IntervalTo<T>> for RangeToInclusive<T> {
    fn from(value: IntervalTo<T>) -> Self {
        ..=(value.end)
    }
}

// IntervalTo<T> vs. IntervalFull

impl<T> Precedes<NonEmpty<IntervalFull>> for NonEmpty<IntervalTo<T>> {}

impl<T> Meets<NonEmpty<IntervalFull>> for NonEmpty<IntervalTo<T>> {}

impl<T> Overlaps<NonEmpty<IntervalFull>> for NonEmpty<IntervalTo<T>> {}

impl<T> Starts<NonEmpty<IntervalFull>> for NonEmpty<IntervalTo<T>> {}

impl<T> Contains<NonEmpty<IntervalFull>> for NonEmpty<IntervalTo<T>> {}

impl<T> Finishes<NonEmpty<IntervalFull>> for NonEmpty<IntervalTo<T>> {}

impl<T> Equals<NonEmpty<IntervalFull>> for NonEmpty<IntervalTo<T>> {}

// IntervalTo<T> vs. IntervalTo

impl<T> Precedes<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalTo<T>> {}

impl<T> Meets<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalTo<T>> {}

impl<T> Overlaps<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalTo<T>> {}

impl<T> Starts<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalTo<T>>
where
    T: PartialOrd,
{
    fn starts(&self, other: &NonEmpty<IntervalTo<T>>) -> bool {
        self.0.end < other.0.end
    }
}

impl<T> Contains<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalTo<T>> {}

impl<T> Finishes<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalTo<T>> {}

impl<T> Equals<NonEmpty<IntervalTo<T>>> for NonEmpty<IntervalTo<T>>
where
    T: PartialOrd,
{
    fn equals(&self, other: &NonEmpty<IntervalTo<T>>) -> bool {
        self == other
    }
}

// IntervalTo<T> vs. IntervalFrom

impl<T> Precedes<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalTo<T>>
where
    T: PartialOrd,
{
    fn precedes(&self, other: &NonEmpty<IntervalFrom<T>>) -> bool {
        self.0.end < other.0.start
    }
}

impl<T> Meets<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalTo<T>>
where
    T: PartialOrd,
{
    fn meets(&self, other: &NonEmpty<IntervalFrom<T>>) -> bool {
        self.0.end == other.0.start
    }
}

impl<T> Overlaps<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalTo<T>>
where
    T: PartialOrd,
{
    fn overlaps(&self, other: &NonEmpty<IntervalFrom<T>>) -> bool {
        self.0.end > other.0.start
    }
}

impl<T> Starts<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalTo<T>> {}

impl<T> Contains<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalTo<T>> {}

impl<T> Finishes<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalTo<T>> {}

impl<T> Equals<NonEmpty<IntervalFrom<T>>> for NonEmpty<IntervalTo<T>> {}

// IntervalTo<T> vs. Interval

impl<T> Precedes<NonEmpty<Interval<T>>> for NonEmpty<IntervalTo<T>>
where
    T: PartialOrd,
{
    fn precedes(&self, other: &NonEmpty<Interval<T>>) -> bool {
        self.0.end < other.0.start
    }
}

impl<T> Meets<NonEmpty<Interval<T>>> for NonEmpty<IntervalTo<T>>
where
    T: PartialOrd,
{
    fn meets(&self, other: &NonEmpty<Interval<T>>) -> bool {
        self.0.end == other.0.start
    }
}

impl<T> Overlaps<NonEmpty<Interval<T>>> for NonEmpty<IntervalTo<T>>
where
    T: PartialOrd,
{
    fn overlaps(&self, other: &NonEmpty<Interval<T>>) -> bool {
        (self.0.end > other.0.start) && (self.0.end < other.0.end)
    }
}

impl<T> Starts<NonEmpty<Interval<T>>> for NonEmpty<IntervalTo<T>> {}

impl<T> Contains<NonEmpty<Interval<T>>> for NonEmpty<IntervalTo<T>>
where
    T: PartialOrd,
{
    fn contains(&self, other: &NonEmpty<Interval<T>>) -> bool {
        self.0.end > other.0.end
    }
}

impl<T> Finishes<NonEmpty<Interval<T>>> for NonEmpty<IntervalTo<T>> {}

impl<T> Equals<NonEmpty<Interval<T>>> for NonEmpty<IntervalTo<T>> {}
